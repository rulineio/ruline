import * as v from 'valibot';

export async function inviteMember(member: InviteMemberForm): Promise<void> {
    const response = await fetch('/invitations', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            email: member.email,
            role: member.role,
            name: `${member.firstName} ${member.lastName}`,
        }),
    });

    if (response.status !== 201) {
        throw new Error('Something went wrong. Please try again later.');
    }
}
export const InviteMemberSchema = v.object({
    email: v.pipe(
        v.string('Please enter a valid email address'),
        v.nonEmpty('Please enter your email address'),
        v.email('Please enter a valid email address'),
    ),
    firstName: v.pipe(
        v.string('Please enter a valid first name'),
        v.nonEmpty('Please enter your first name'),
        v.trim(),
    ),
    lastName: v.optional(v.pipe(v.string(), v.trim())),
    role: v.optional(v.picklist(['admin', 'editor', 'viewer']), 'viewer'),
});
export type InviteMemberForm = v.InferInput<typeof InviteMemberSchema>;
