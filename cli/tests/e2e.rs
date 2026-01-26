#[path = "common/mod.rs"]
mod test_utils;

#[path = "e2e/simple_math.rs"]
mod simple_math;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/console_log.rs"]
mod console_log;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/functions.rs"]
mod functions;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/control_flow.rs"]
mod control_flow;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/arrays.rs"]
mod arrays;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/conditionals.rs"]
mod conditionals;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/arrow_functions.rs"]
mod arrow_functions;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/binary.rs"]
mod binary;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/new_nodes.rs"]
mod new_nodes;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/heavy_bench.rs"]
mod heavy_bench;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/tests_from_inputs.rs"]
mod tests_from_inputs;
