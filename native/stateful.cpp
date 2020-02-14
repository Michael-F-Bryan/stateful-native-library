// This is deliberately meant to look like crap. I don't normally write like
// this, I promise 👼

#include "stateful.h"
#include <vector>
#include <unordered_map>
#include <string>
#include <variant>
#include <stdint.h>
#include <memory>

enum class State
{
    Uninitialized,
    Initialized,
    SettingParameters,
    AddingInputs,
    AddingGroup,
    Executing,
};

class Item
{
public:
    virtual void flatten(std::vector<int32_t> &dest) = 0;
    virtual ~Item() {}
};

class Group : public Item
{
private:
    std::unordered_map<std::string, int32_t> items;

public:
    void insert(std::string key, int32_t value)
    {
        items.insert(std::make_pair(key, value));
    }

    void flatten(std::vector<int32_t> &dest)
    {
        for (auto &&pair : items)
        {
            dest.push_back(pair.second);
        }
    }
};

class SingleItem : public Item
{
private:
    int32_t value;

public:
    SingleItem(int32_t v) : value(v) {}

    void flatten(std::vector<int32_t> &dest)
    {
        dest.push_back(value);
    }
};

using Parameter = std::variant<int32_t, bool>;

// our actual global variables

State current_state = State::Uninitialized;
std::unordered_map<std::string, std::unique_ptr<Item>> *inputs;
std::unordered_map<std::string, Parameter> *parameters;

std::vector<int32_t> *temp_results;
std::string *temp_group_name;
Group *temp_group;

int stateful_open()
{
    if (current_state != State::Uninitialized)
    {
        return RESULT_BAD_STATE;
    }

    inputs = new std::unordered_map<std::string, std::unique_ptr<Item>>();
    parameters = new std::unordered_map<std::string, Parameter>();
    current_state = State::Initialized;

    return RESULT_OK;
}

int stateful_close()
{
    if (inputs)
    {
        delete inputs;
        inputs = nullptr;
    }
    if (parameters)
    {
        delete parameters;
        parameters = nullptr;
    }
    current_state = State::Uninitialized;
    return RESULT_OK;
}

int stateful_start_setting_parameters()
{
    if (current_state != State::Initialized)
    {
        return RESULT_BAD_STATE;
    }

    current_state = State::SettingParameters;
    return RESULT_OK;
}

template <typename T>
static int set_parameter(const char *name, T value)
{
    if (current_state != State::SettingParameters)
    {
        return RESULT_BAD_STATE;
    }
    if (!name)
    {
        return RESULT_INVALID_ARGUMENT;
    }

    parameters->insert(std::make_pair(name, value));

    return RESULT_OK;
}

int stateful_set_bool_var(const char *name, bool value)
{
    return set_parameter(name, value);
}

int stateful_set_int_var(const char *name, int value)
{
    return set_parameter(name, value);
}

int stateful_end_setting_parameters()
{
    if (current_state != State::SettingParameters)
    {
        return RESULT_BAD_STATE;
    }

    current_state = State::Initialized;
    return RESULT_OK;
}

int stateful_start_adding_items()
{
    if (current_state != State::Initialized)
    {
        return RESULT_BAD_STATE;
    }

    current_state = State::AddingInputs;
    return RESULT_OK;
}

template <typename T>
int add_input(std::string name, const T value)
{
    if (current_state != State::AddingInputs)
    {
        return RESULT_BAD_STATE;
    }

    inputs->insert(std::pair(name, std::make_unique<T>(value)));
    return RESULT_OK;
}

int stateful_add_item(const char *name, int value)
{
    return add_input(name, SingleItem(value));
}

int stateful_start_adding_group(const char *name)
{
    if (current_state != State::AddingInputs)
    {
        return RESULT_BAD_STATE;
    }

    temp_group_name = new std::string(name);
    temp_group = new Group();

    current_state = State::AddingGroup;
    return RESULT_OK;
}

int stateful_add_group_item(const char *name, int value)
{
    if (current_state != State::AddingGroup)
    {
        return RESULT_BAD_STATE;
    }

    temp_group->insert(name, value);
    return RESULT_OK;
}

int stateful_end_adding_group()
{
    if (current_state != State::AddingGroup)
    {
        return RESULT_BAD_STATE;
    }
    current_state = State::AddingInputs;

    add_input(*temp_group_name, *temp_group);

    delete temp_group;
    temp_group = nullptr;
    delete temp_group_name;
    temp_group_name = nullptr;

    return RESULT_OK;
}

int stateful_end_adding_items()
{
    if (current_state != State::AddingInputs)
    {
        return RESULT_BAD_STATE;
    }

    current_state = State::Initialized;
    return RESULT_OK;
}

// This overload stuff legitimately feels like magic...
// https://www.bfilipek.com/2018/06/variant.html#visitors-for-stdvariant
template <class... Ts>
struct overload : Ts...
{
    using Ts::operator()...;
};
template <class... Ts>
overload(Ts...)->overload<Ts...>;

int stateful_execute(progress_cb progress, result_cb result)
{
    if (current_state != State::Initialized)
    {
        return RESULT_BAD_STATE;
    }
    current_state = State::Executing;

    std::vector<int32_t> results;
    int i = 0;

    for (auto &pair : *inputs)
    {
        pair.second->flatten(results);

        double percent = 100.0 * i / inputs->size();
        progress((int)percent);

        i++;
    }
    progress(100);

    temp_results = &results;
    result(results.size());
    temp_results = nullptr;

    current_state = State::Initialized;
    return RESULT_OK;
}

int stateful_get_num_outputs(int *value)
{
    if (current_state != State::Executing)
    {
        return RESULT_BAD_STATE;
    }

    *value = temp_results->size();
    return RESULT_OK;
}

int stateful_get_output_by_index(int index, int *value)
{
    if (current_state != State::Executing)
    {
        return RESULT_BAD_STATE;
    }

    auto &results = *temp_results;

    if ((std::size_t)index >= results.size())
    {
        return RESULT_INVALID_ARGUMENT;
    }

    *value = results[index];
    return RESULT_OK;
}
