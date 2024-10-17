import * as v from 'valibot';

export async function login(req: LoginForm): Promise<void> {
    const response = await fetch('/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email: req.email }),
    });

    if (response.status !== 202) {
        throw new Error('Something went wrong. Please try again later.');
    }
}

export const LoginSchema = v.object({
    email: v.pipe(
        v.string('Please enter a valid email address'),
        v.nonEmpty('Please enter your email address'),
        v.email('Please enter a valid email address'),
    ),
});
export type LoginForm = v.InferInput<typeof LoginSchema>;
