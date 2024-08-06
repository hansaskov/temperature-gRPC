import type { InferSelectModel } from 'drizzle-orm';
import { pgTable, real, serial, text, timestamp } from 'drizzle-orm/pg-core';

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

export const conditionsTable = pgTable("conditions", {
	time: timestamp('time').notNull(),
	temperature: real('temperature').notNull()
})

export type SelectCondition = typeof conditionsTable.$inferSelect
export type InsertCondition = typeof conditionsTable.$inferInsert
