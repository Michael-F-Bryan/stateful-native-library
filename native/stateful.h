#if !defined(STATEFUL_H)
#define STATEFUL_H

#include <stdbool.h>

// The various possible return codes.
enum
{
    // The function completed successfully.
    RESULT_OK,
    // A function was called out of order.
    RESULT_BAD_STATE,
    // One of the provided arguments is invalid.
    RESULT_INVALID_ARGUMENT,
};

// Initialize the library. MUST be run before any other function.
int stateful_open();

// Clean up any state associated with this library.
int stateful_close();

// Begin setting parameters. MUST be run before any parameters can be set.
int stateful_start_setting_parameters();
int stateful_set_bool_var(const char *name, bool value);
int stateful_set_int_var(const char *name, int value);
// Finish setting parameters.
int stateful_end_setting_parameters();

// Start adding input items.
int stateful_start_adding_items();
// Add a single item as an input.
int stateful_add_item(const char *name, int value);
// Start adding a group of items.
int stateful_start_adding_group(const char *name);
// Add an item to the current group. stateful_start_adding_group MUST be called
// beforehand.
int stateful_add_group_item(const char *name, int value);
// Finish adding items to the current group, adding the overall group to the
// list of inputs.
int stateful_end_adding_group();
// Finish setting up the list of inputs.
int stateful_end_adding_items();

// A callback used to notify the caller when progress is made.
typedef int (*progress_cb)(int percent);
// A callback used to let the user retrieve results.
typedef int (*result_cb)(int number_of_results);

// Run the code.
int stateful_execute(progress_cb progress, result_cb result);

// Try to get the number of outputs in the result.
int stateful_get_num_outputs(int *value);
// Tries to retrieve a particular output.
int stateful_get_output_by_index(int index, int *value);

#endif // STATEFUL_H
