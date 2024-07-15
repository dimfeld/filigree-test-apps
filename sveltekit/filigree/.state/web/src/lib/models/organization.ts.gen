import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export type OrganizationId = string;

export const OrganizationSchema = z.object({
	id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
	owner: z.string().uuid().optional(),
	default_role: z.string().uuid().optional(),
});

export type Organization = z.infer<typeof OrganizationSchema>;
export const OrganizationListResultSchema = OrganizationSchema;
export type OrganizationListResult = Organization;
export const OrganizationCreateResultSchema = OrganizationSchema;
export type OrganizationCreateResult = Organization;
export const OrganizationSchema = OrganizationSchema;
export type Organization = Organization;

export const OrganizationOwnerViewSchema = z.object({
	id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
	owner: z.string().uuid().optional(),
	default_role: z.string().uuid().optional(),
});

export type OrganizationOwnerView = z.infer<typeof OrganizationOwnerViewSchema>;

export const OrganizationUserViewSchema = z.object({
	id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	name: z.string(),
});

export type OrganizationUserView = z.infer<typeof OrganizationUserViewSchema>;

export const baseUrl = "organizations";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const OrganizationModel: ModelDefinition<typeof OrganizationSchema> = {
	name: "Organization",
	plural: "Organizations",
	baseUrl,
	urls,
	schema: OrganizationSchema,
	createSchema: OrganizationCreatePayloadSchema,
	updateSchema: OrganizationUpdatePayloadSchema,
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
			name: "owner",
			type: "uuid",
			label: "Owner",
			constraints: {
				required: false,
			},
		},
		{
			name: "default_role",
			type: "uuid",
			label: "Default Role",
			constraints: {
				required: false,
			},
		},
		{
			name: "active",
			type: "boolean",
			label: "Active",
			constraints: {
				required: true,
			},
		},
	],
};
