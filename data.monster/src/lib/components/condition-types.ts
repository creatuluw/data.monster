export type Operator = "eq" | "neq" | "gt" | "gte" | "lt" | "lte" | "contains" | "not_contains" | "is_null" | "is_not_null";

export interface SimpleCondition {
  field: string;
  operator: Operator;
  value: string | number | null;
}

export interface ConditionGroup {
  logic: "AND" | "OR";
  conditions: (SimpleCondition | ConditionGroup)[];
}