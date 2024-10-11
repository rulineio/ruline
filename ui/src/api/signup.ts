import * as v from 'valibot';

export async function signup(req: SignupForm): Promise<void> {
    const response = await fetch('/signup', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            email: req.email,
            name: `${req.firstName} ${req.lastName}`.trim(),
        }),
    });

    if (response.status !== 202) {
        throw new Error('Something went wrong . Please try again later.');
    }
}

export const SignupSchema = v.object({
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
});

export type SignupForm = v.InferInput<typeof SignupSchema>;
