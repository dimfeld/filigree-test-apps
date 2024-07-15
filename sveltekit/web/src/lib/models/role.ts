import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export type RoleId = string;

export const RoleSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
	description: z.string().optional(),
});

export type Role = z.infer<typeof RoleSchema>;
export const RoleUserViewSchema = RoleSchema;
export type RoleUserView = Role;
export const RoleOwnerViewSchema = RoleSchema;
export type RoleOwnerView = Role;
export const RoleListResultSchema = RoleSchema;
export type RoleListResult = Role;
export const RoleCreateResultSchema = RoleSchema;
export type RoleCreateResult = Role;
export const RoleSchema = RoleSchema;
export type Role = Role;

export const baseUrl = "roles";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const RoleModel: ModelDefinition<typeof RoleSchema> = {
	name: "Role",
	plural: "Roles",
	baseUrl,
	urls,
	schema: RoleSchema,
	createSchema: RoleCreatePayloadSchema,
	updateSchema: RoleUpdatePayloadSchema,
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
				required: true,
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
			name: "description",
			type: "text",
			label: "Description",
			constraints: {
				required: false,
			},
		},
	],
};
