import type { InferSelectModel } from 'drizzle-orm';
import { pgTable, real, text, timestamp } from 'drizzle-orm/pg-core';
import { timestamptz } from '@/lib/db/timescale'

export type User = InferSelectModel<typeof userTable>
export const userTable = pgTable('user', {
  id: text('id').primaryKey(),
  githubId: text('github_id').unique(),
  username: text('username').notNull(),
});


export type Session = InferSelectModel<typeof sessionTable>
export const sessionTable = pgTable("session", {
	id: text("id").primaryKey(),
	userId: text("user_id")
		.notNull()
		.references(() => userTable.id),
	expiresAt: timestamp("expires_at", {
		withTimezone: true,
		mode: "date"
	}).notNull()
});


export const conditions = pgTable("conditions", {
	time: timestamptz('time').notNull(),
	cpu_temperature: real('cpu_temperature').notNull(),
	cpu_usage: real('cpu_usage').notNull(),
	memory_usage: real('memory_usage').notNull()

})

export type SelectCondition = typeof conditions.$inferSelect
export type InsertCondition = typeof conditions.$inferInsert



