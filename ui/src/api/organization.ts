import * as v from 'valibot';

export async function fetchOrganization(): Promise<Organization> {
    const response = await fetch('/organizations');

    if (response.status !== 200) {
        throw new Error(`Error fetching organization: ${response.statusText}`);
    }
    const data = await response.json();
    return v.parse(Organization, data);
}

export async function createOrganization(
    organization: CreateOrganizationForm,
): Promise<CreateOrganizationResponse> {
    const response = await fetch('/organizations', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(organization),
    });

    if (response.status !== 201) {
        throw new Error('Something went wrong. Please try again later.');
    }

    const data = await response.json();
    return v.parse(CreateOrganizationResponse, data);
}

export async function fetchOrganizationMembers(): Promise<OrganizationMembers> {
    const response = await fetch('/organizations/members');

    if (response.status !== 200) {
        throw new Error(
            `Error fetching organization members: ${response.statusText}`,
        );
    }

    const data = await response.json();
    return v.parse(OrganizationMembers, data);
}

export const Organization = v.object({
    id: v.string(),
    name: v.string(),
    status: v.picklist(['active']),
    logo: v.string(),
});
export type Organization = v.InferInput<typeof Organization>;

export const CreateOrganizationSchema = v.object({
    name: v.pipe(
        v.string('Please enter a valid name'),
        v.nonEmpty('Please enter a name'),
        v.trim(),
    ),
});
export type CreateOrganizationForm = v.InferInput<
    typeof CreateOrganizationSchema
>;

export const CreateOrganizationResponse = v.object({
    project_id: v.string(),
});
export type CreateOrganizationResponse = v.InferInput<
    typeof CreateOrganizationResponse
>;

export const OrganizationMembers = v.array(
    v.object({
        name: v.string(),
        email: v.pipe(v.string(), v.email()),
        avatar: v.string(),
        role: v.picklist(['owner', 'admin', 'editor', 'viewer', 'member']),
        status: v.picklist([
            'active',
            'left',
            'removed',
            'invited',
            'declined',
        ]),
    }),
);
export type OrganizationMembers = v.InferInput<typeof OrganizationMembers>;
