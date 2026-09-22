/**
 * Curated DuckDB function catalog for the master-item expression editor.
 * Snippets use `§` to mark where the caret lands after insertion (removed
 * from the inserted text). Entries lean toward the dialect features that make
 * master items powerful: aggregates + FILTER, window functions, date math,
 * casts — the Qlik-Sense-style building blocks.
 */

export type FuncCat = 'agg' | 'win' | 'date' | 'str' | 'math' | 'cond' | 'cast';

export type FuncDef = {
	/** display name (first word of sig) */
	name: string;
	/** signature shown in the picker, e.g. `date_trunc(part, timestamp)` */
	sig: string;
	/** one-line doc */
	doc: string;
	cat: FuncCat;
	/** text inserted on accept; `§` = caret position */
	snippet: string;
};

export const CAT_LABELS: Record<FuncCat, string> = {
	agg: 'Aggregates',
	win: 'Window',
	date: 'Date & time',
	str: 'String',
	math: 'Math',
	cond: 'Conditional',
	cast: 'Cast'
};

export const DUCKDB_FUNCTIONS: FuncDef[] = [
	// ── aggregates ────────────────────────────────────────────────────────────
	{ name: 'sum', sig: 'sum(x)', doc: 'Total across rows (ignores NULLs).', cat: 'agg', snippet: 'sum(§)' },
	{ name: 'avg', sig: 'avg(x)', doc: 'Arithmetic mean.', cat: 'agg', snippet: 'avg(§)' },
	{ name: 'min', sig: 'min(x)', doc: 'Smallest value.', cat: 'agg', snippet: 'min(§)' },
	{ name: 'max', sig: 'max(x)', doc: 'Largest value.', cat: 'agg', snippet: 'max(§)' },
	{ name: 'count', sig: 'count(x) · count(*) · count(distinct x)', doc: 'Row / value / distinct count.', cat: 'agg', snippet: 'count(distinct §)' },
	{ name: 'median', sig: 'median(x)', doc: 'Middle value (50% quantile).', cat: 'agg', snippet: 'median(§)' },
	{ name: 'quantile_cont', sig: 'quantile_cont(x, p)', doc: 'Interpolated percentile, p in 0…1 (0.5 = median).', cat: 'agg', snippet: 'quantile_cont(§, 0.9)' },
	{ name: 'quantile_disc', sig: 'quantile_disc(x, p)', doc: 'Percentile returning an existing value.', cat: 'agg', snippet: 'quantile_disc(§, 0.9)' },
	{ name: 'mode', sig: 'mode(x)', doc: 'Most frequent value.', cat: 'agg', snippet: 'mode(§)' },
	{ name: 'stddev', sig: 'stddev(x)', doc: 'Sample standard deviation.', cat: 'agg', snippet: 'stddev(§)' },
	{ name: 'stddev_pop', sig: 'stddev_pop(x)', doc: 'Population standard deviation.', cat: 'agg', snippet: 'stddev_pop(§)' },
	{ name: 'var', sig: 'var(x)', doc: 'Sample variance.', cat: 'agg', snippet: 'var(§)' },
	{ name: 'string_agg', sig: 'string_agg(x, sep)', doc: 'Concatenate strings with a separator.', cat: 'agg', snippet: "string_agg(§, ', ')" },
	{ name: 'list', sig: 'list(x)', doc: 'Collect values into a list.', cat: 'agg', snippet: 'list(§)' },
	{ name: 'arg_max', sig: 'arg_max(val, order)', doc: "Value of val where order is greatest (e.g. latest amount).", cat: 'agg', snippet: 'arg_max(val, order)' },
	{ name: 'arg_min', sig: 'arg_min(val, order)', doc: 'Value of val where order is smallest.', cat: 'agg', snippet: 'arg_min(val, order)' },
	{ name: 'first', sig: 'first(x)', doc: 'First value in the group.', cat: 'agg', snippet: 'first(§)' },
	{ name: 'last', sig: 'last(x)', doc: 'Last value in the group.', cat: 'agg', snippet: 'last(§)' },
	{ name: 'FILTER', sig: 'agg(x) filter (where cond)', doc: 'Aggregate only rows matching cond — conditional totals without CASE.', cat: 'agg', snippet: 'sum(col) filter (where §)' },

	// ── window ────────────────────────────────────────────────────────────────
	{ name: 'over', sig: 'agg(x) over (partition by … order by …)', doc: 'Window aggregate — e.g. share of total, running sum.', cat: 'win', snippet: 'sum(§) over ()' },
	{ name: 'row_number', sig: 'row_number() over (…)', doc: '1-based row counter within the window.', cat: 'win', snippet: 'row_number() over (partition by pk order by §)' },
	{ name: 'rank', sig: 'rank() over (…)', doc: 'Rank with gaps (1, 1, 3…).', cat: 'win', snippet: 'rank() over (order by § desc)' },
	{ name: 'dense_rank', sig: 'dense_rank() over (…)', doc: 'Rank without gaps (1, 1, 2…).', cat: 'win', snippet: 'dense_rank() over (order by § desc)' },
	{ name: 'percent_rank', sig: 'percent_rank() over (…)', doc: 'Relative rank 0…1 within the window.', cat: 'win', snippet: 'percent_rank() over (order by §)' },
	{ name: 'ntile', sig: 'ntile(n) over (…)', doc: 'Bucket number 1…n (quartiles, deciles).', cat: 'win', snippet: 'ntile(4) over (order by §)' },
	{ name: 'lag', sig: 'lag(x, offset) over (…)', doc: 'Value from a previous row — period-over-period deltas.', cat: 'win', snippet: 'lag(§, 1) over (order by sort_col)' },
	{ name: 'lead', sig: 'lead(x, offset) over (…)', doc: 'Value from a following row.', cat: 'win', snippet: 'lead(§, 1) over (order by sort_col)' },
	{ name: 'first_value', sig: 'first_value(x) over (…)', doc: 'First value in the window frame.', cat: 'win', snippet: 'first_value(§) over (partition by grp order by sort_col)' },
	{ name: 'last_value', sig: 'last_value(x) over (…)', doc: 'Last value in the window frame.', cat: 'win', snippet: 'last_value(§) over (order by sort_col)' },

	// ── date & time ───────────────────────────────────────────────────────────
	{ name: 'date_trunc', sig: "date_trunc(part, ts)", doc: 'Truncate to day / week / month / year… — grouping periods.', cat: 'date', snippet: "date_trunc('month', §)" },
	{ name: 'date_part', sig: "date_part(part, ts)", doc: "Extract a component: 'year', 'month', 'dow', 'hour'…", cat: 'date', snippet: "date_part('year', §)" },
	{ name: 'date_diff', sig: "date_diff(part, a, b)", doc: "Difference in 'day' / 'month' / 'second'… units.", cat: 'date', snippet: "date_diff('day', start_col, §)" },
	{ name: 'interval', sig: "ts + interval 'n units'", doc: 'Offset a date/timestamp — rolling windows, maturity dates.', cat: 'date', snippet: "§ + interval '30 days'" },
	{ name: 'strftime', sig: 'strftime(ts, fmt)', doc: "Format a timestamp — '%Y-%m' gives '2026-09'.", cat: 'date', snippet: "strftime(§, '%Y-%m')" },
	{ name: 'strptime', sig: 'strptime(s, fmt)', doc: 'Parse a string into a timestamp.', cat: 'date', snippet: "strptime(§, '%d/%m/%Y')" },
	{ name: 'epoch', sig: 'epoch(ts)', doc: 'Seconds since 1970-01-01.', cat: 'date', snippet: 'epoch(§)' },
	{ name: 'epoch_ms', sig: 'epoch_ms(ts)', doc: 'Milliseconds since 1970-01-01.', cat: 'date', snippet: 'epoch_ms(§)' },
	{ name: 'today', sig: 'today()', doc: 'Current date (statement-stable).', cat: 'date', snippet: 'today()' },
	{ name: 'now', sig: 'now()', doc: 'Current timestamp.', cat: 'date', snippet: 'now()' },
	{ name: 'monthname', sig: 'monthname(d)', doc: "'Jan'…'Dec' — friendly month label.", cat: 'date', snippet: 'monthname(§)' },
	{ name: 'dayname', sig: 'dayname(d)', doc: "'Monday'…'Sunday'.", cat: 'date', snippet: 'dayname(§)' },
	{ name: 'last_day', sig: 'last_day(d)', doc: 'Last day of the month.', cat: 'date', snippet: 'last_day(§)' },

	// ── string ────────────────────────────────────────────────────────────────
	{ name: 'upper', sig: 'upper(s)', doc: 'UPPERCASE.', cat: 'str', snippet: 'upper(§)' },
	{ name: 'lower', sig: 'lower(s)', doc: 'lowercase — normalize labels.', cat: 'str', snippet: 'lower(§)' },
	{ name: 'initcap', sig: 'initcap(s)', doc: 'Title Case Each Word.', cat: 'str', snippet: 'initcap(§)' },
	{ name: 'length', sig: 'length(s)', doc: 'String length in characters.', cat: 'str', snippet: 'length(§)' },
	{ name: 'trim', sig: 'trim(s)', doc: 'Strip surrounding whitespace.', cat: 'str', snippet: 'trim(§)' },
	{ name: 'concat', sig: 'concat(a, b, …)', doc: 'Join strings.', cat: 'str', snippet: "concat(§, '')" },
	{ name: 'concat_ws', sig: "concat_ws(sep, a, b, …)", doc: 'Join with separator, skipping NULLs.', cat: 'str', snippet: "concat_ws(' — ', §)" },
	{ name: '||', sig: 'a || b', doc: 'String concatenation operator.', cat: 'str', snippet: "§ || ''" },
	{ name: 'contains', sig: 'contains(s, needle)', doc: 'TRUE if s contains needle.', cat: 'str', snippet: "contains(§, 'x')" },
	{ name: 'substring', sig: 'substring(s, start, len)', doc: '1-based slice of the string.', cat: 'str', snippet: 'substring(§, 1, 10)' },
	{ name: 'left', sig: 'left(s, n)', doc: 'First n characters.', cat: 'str', snippet: 'left(§, 3)' },
	{ name: 'right', sig: 'right(s, n)', doc: 'Last n characters.', cat: 'str', snippet: 'right(§, 3)' },
	{ name: 'replace', sig: 'replace(s, from, to)', doc: 'Replace all occurrences.', cat: 'str', snippet: "replace(§, 'x', 'y')" },
	{ name: 'regexp_matches', sig: 'regexp_matches(s, pat)', doc: 'TRUE if the pattern matches anywhere.', cat: 'str', snippet: "regexp_matches(§, '^A')" },
	{ name: 'regexp_extract', sig: 'regexp_extract(s, pat)', doc: 'First capture-group match as string.', cat: 'str', snippet: "regexp_extract(§, '\\d+')" },
	{ name: 'split_part', sig: "split_part(s, sep, n)", doc: 'nth piece after splitting.', cat: 'str', snippet: "split_part(§, '-', 1)" },
	{ name: 'lpad', sig: 'lpad(s, n, fill)', doc: 'Pad on the left — zero-pad codes.', cat: 'str', snippet: "lpad(§, 6, '0')" },

	// ── math ──────────────────────────────────────────────────────────────────
	{ name: 'round', sig: 'round(x, digits)', doc: 'Round to n decimal places.', cat: 'math', snippet: 'round(§, 2)' },
	{ name: 'ceil', sig: 'ceil(x)', doc: 'Round up.', cat: 'math', snippet: 'ceil(§)' },
	{ name: 'floor', sig: 'floor(x)', doc: 'Round down — numeric bucketing.', cat: 'math', snippet: 'floor(§)' },
	{ name: 'abs', sig: 'abs(x)', doc: 'Absolute value.', cat: 'math', snippet: 'abs(§)' },
	{ name: 'pow', sig: 'pow(x, y)', doc: 'x to the power y.', cat: 'math', snippet: 'pow(§, 2)' },
	{ name: 'sqrt', sig: 'sqrt(x)', doc: 'Square root.', cat: 'math', snippet: 'sqrt(§)' },
	{ name: 'exp', sig: 'exp(x)', doc: 'e^x.', cat: 'math', snippet: 'exp(§)' },
	{ name: 'ln', sig: 'ln(x)', doc: 'Natural log — log-scale ratios.', cat: 'math', snippet: 'ln(§)' },
	{ name: 'log10', sig: 'log10(x)', doc: 'Base-10 log.', cat: 'math', snippet: 'log10(§)' },
	{ name: 'mod', sig: 'mod(a, b)', doc: 'Remainder of a / b.', cat: 'math', snippet: 'mod(§, 10)' },
	{ name: 'sign', sig: 'sign(x)', doc: '-1, 0 or 1.', cat: 'math', snippet: 'sign(§)' },
	{ name: 'greatest', sig: 'greatest(a, b, …)', doc: 'Largest of the arguments.', cat: 'math', snippet: 'greatest(§, 0)' },
	{ name: 'least', sig: 'least(a, b, …)', doc: 'Smallest of the arguments.', cat: 'math', snippet: 'least(§, 0)' },

	// ── conditional ───────────────────────────────────────────────────────────
	{ name: 'case', sig: 'case when c then v … else v end', doc: 'Conditional mapping — bucketing, label remapping.', cat: 'cond', snippet: 'case when cond then § else null end' },
	{ name: 'if', sig: 'if(cond, a, b)', doc: 'Inline conditional (DuckDB shorthand).', cat: 'cond', snippet: 'if(cond, §, 0)' },
	{ name: 'coalesce', sig: 'coalesce(a, b, …)', doc: 'First non-NULL argument — defaulting.', cat: 'cond', snippet: 'coalesce(§, 0)' },
	{ name: 'ifnull', sig: 'ifnull(a, b)', doc: 'Two-argument coalesce.', cat: 'cond', snippet: 'ifnull(§, 0)' },
	{ name: 'nullif', sig: 'nullif(a, b)', doc: 'NULL when a = b — e.g. divide-by-zero guards.', cat: 'cond', snippet: 'nullif(§, 0)' },
	{ name: 'in', sig: "x in ('a', 'b', …)", doc: 'Membership test.', cat: 'cond', snippet: "§ in ('a', 'b')" },
	{ name: 'try', sig: 'try(expr)', doc: 'NULL instead of an error on failure.', cat: 'cond', snippet: 'try(§)' },

	// ── cast ──────────────────────────────────────────────────────────────────
	{ name: 'cast', sig: 'cast(x as type)', doc: 'Explicit type conversion.', cat: 'cast', snippet: 'cast(§ as double)' },
	{ name: '::', sig: 'x::type', doc: 'Postfix cast — amount::int, ts::date.', cat: 'cast', snippet: '§::varchar' },
	{ name: 'try_cast', sig: 'try_cast(x as type)', doc: 'Cast returning NULL when not convertible.', cat: 'cast', snippet: 'try_cast(§ as int)' }
];
