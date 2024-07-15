import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export type UserId = string;

export const UserSchema = z.object({
	id: z.string(),
	organization_id: z.string().optional(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
	email: z.string().optional(),
	avatar_url: z.string().optional(),
});

export type User = z.infer<typeof UserSchema>;
export const UserUserViewSchema = UserSchema;
export type UserUserView = User;
export const UserOwnerViewSchema = UserSchema;
export type UserOwnerView = User;
export const UserListResultSchema = UserSchema;
export type UserListResult = User;
export const UserCreateResultSchema = UserSchema;
export type UserCreateResult = User;
export const UserSchema = UserSchema;
export type User = User;

export const baseUrl = "users";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const UserModel: ModelDefinition<typeof UserSchema> = {
	name: "User",
	plural: "Users",
	baseUrl,
	urls,
	schema: UserSchema,
	createSchema: UserCreatePayloadSchema,
	updateSchema: UserUpdatePayloadSchema,
	fields: [
		{
			name: "id",
			type: "uuid",
			label: "Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "organization_id",
			type: "uuid",
			label: "Organization Id",
			constraints: {
				required: false,
			},
		},
		{
			name: "updated_at",
			type: "date-time",
			label: "Updated At",
			constraints: {
				required: true,
			},
		},
		{
			name: "created_at",
			type: "date-time",
			label: "Created At",
			constraints: {
				required: true,
			},
		},
		{
			name: "name",
			type: "text",
			label: "Name",
			constraints: {
				required: true,
			},
		},
		{
			name: "password_hash",
			type: "text",
			label: "Password Hash",
			constraints: {
				required: false,
			},
		},
		{
			name: "email",
			type: "text",
			label: "Email",
			constraints: {
				required: false,
			},
		},
		{
			name: "avatar_url",
			type: "text",
			label: "Avatar Url",
			constraints: {
				required: false,
			},
		},
	],
};
