// A demo program to make sure the bindings work.

#include <assert.h>
#include <iostream>
#include "stateful.h"

void handle_error(int result, const char *cause);
int on_progress(int percent);
int on_finished(int num_results);

#define question_mark(expr)           \
    do                                \
    {                                 \
        auto got = expr;              \
        if (got != RESULT_OK)         \
        {                             \
            handle_error(got, #expr); \
        }                             \
    } while (0);

int main()
{
    question_mark(stateful_open());

    question_mark(stateful_start_setting_parameters());
    question_mark(stateful_set_bool_var("first", true));
    question_mark(stateful_set_int_var("second", 42));
    question_mark(stateful_end_setting_parameters());

    question_mark(stateful_start_adding_items());
    question_mark(stateful_add_item("thing", 1));
    question_mark(stateful_add_item("another thing", 2));
    question_mark(stateful_start_adding_group("group"));
    question_mark(stateful_add_group_item("nested", 5));
    question_mark(stateful_end_adding_group());
    question_mark(stateful_end_adding_items());

    question_mark(stateful_execute(on_progress, on_finished));

    question_mark(stateful_close());
}

void handle_error(int result, const char *cause)
{
    switch (result)
    {
    case RESULT_OK:
        return;
    case RESULT_BAD_STATE:
        std::cout << "Bad State at " << cause << std::endl;
        break;
    case RESULT_INVALID_ARGUMENT:
        std::cout << "Invalid Argument at " << cause << std::endl;
        break;
    default:
        std::cout << "Unknown error " << result << " at " << cause << std::endl;
    }

    exit(result);
}

int on_progress(int percent)
{
    std::cout << "Progress " << percent << "%" << std::endl;
    return RESULT_OK;
}

int on_finished(int num_results)
{
    std::cout << "Finished with " << num_results << " items" << std::endl;
    return RESULT_OK;
}