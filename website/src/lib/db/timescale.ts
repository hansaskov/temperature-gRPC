import { Column, is, sql, type AnyColumn, type SQL, type SQLWrapper } from 'drizzle-orm';
import { customType } from 'drizzle-orm/pg-core';

export const timestamptz = customType<
  {
    data: Date;
    driverData: string;
    config: { withTimezone: boolean; precision?: number };
  }
>({
  dataType(config) {
    const precision = typeof config?.precision !== 'undefined'
      ? ` (${config.precision})`
      : '';
    return `timestamptz${precision}${
      config?.withTimezone ? ' with time zone' : ''
    }`;
  },
  fromDriver(value: string): Date {
    return new Date(value);
  },
});

type ValidUnits = 'microseconds' | 'milliseconds' | 'second' | 'seconds' | 'minute' | 'minutes' | 'hour' | 'hours' | 'day' | 'days' | 'week' | 'weeks' | 'month' | 'months' | 'year' | 'years';

type IntervalString = `${number} ${ValidUnits}`;

// Define the time_bucket function with type-safe interval argument
export function time_bucket<T extends SQLWrapper & { _ : { data: Date } }>(
    expression: T,
    interval: IntervalString,
): SQL<Date> {
    return sql`time_bucket(${interval}, ${expression})`.mapWith((v) => new Date(v));
}