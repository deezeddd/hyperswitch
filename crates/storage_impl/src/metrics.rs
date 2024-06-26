use router_env::{counter_metric, global_meter, metrics_context};

metrics_context!(CONTEXT);
global_meter!(GLOBAL_METER, "ROUTER_API");

counter_metric!(KV_MISS, GLOBAL_METER); // No. of KV misses

// Metrics for KV
counter_metric!(KV_OPERATION_SUCCESSFUL, GLOBAL_METER);
counter_metric!(KV_OPERATION_FAILED, GLOBAL_METER);
counter_metric!(KV_PUSHED_TO_DRAINER, GLOBAL_METER);
counter_metric!(KV_FAILED_TO_PUSH_TO_DRAINER, GLOBAL_METER);
counter_metric!(KV_SOFT_KILL_ACTIVE_UPDATE, GLOBAL_METER);
