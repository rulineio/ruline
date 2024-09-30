import { createFileRoute, redirect } from '@tanstack/react-router';
import * as v from 'valibot';
import { useForm } from 'react-hook-form';
import { login, type LoginForm, LoginSchema } from '../api/login';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { create } from 'zustand';
import { AuthForm } from '../components/AuthForm';
import { Input } from '../components/Input';

export const Route = createFileRoute('/login')({
    component: Login,
    beforeLoad: async ({ context }) => {
        const { refetch } = context.auth;
        const { data: user } = await refetch();
        if (user) {
            throw redirect({ to: '/' });
        }
    },
});

const useLinkSentStore = create<{
    linkSent: boolean;
    email: string;
    setLink: () => void;
    setEmail: (email: string) => void;
}>((set) => ({
    linkSent: false,
    email: '',
    setLink: () => set({ linkSent: true }),
    setEmail: (email) => set({ email }),
}));

function Login() {
    const {
        register,
        handleSubmit,
        setError,
        formState: { errors, disabled },
    } = useForm<LoginForm>({
        resolver: valibotResolver(LoginSchema),
    });

    const { linkSent, setLink, email, setEmail } = useLinkSentStore();

    const submit = async (data: LoginForm) => {
        try {
            await login(data);
            setLink();
            setEmail(data.email);
        } catch (e: unknown) {
            if (e instanceof Error) {
                setError('root', {
                    type: 'manual',
                    message: e.message,
                });
            }
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-blue-800">
            <AuthForm
                title="Login"
                subtitle={{
                    text: 'Don’t have an account?',
                    link: { text: 'Sign up', href: '/signup' },
                }}
                linkSent={linkSent}
                email={email}
                oauth={{
                    google: { enabled: true, text: 'Sign in with Google' },
                }}
                onSubmit={handleSubmit(submit)}
                button={{ text: 'Sign In', disabled }}
                error={errors.root?.message}
            >
                <div className="mb-6">
                    <Input
                        name="email"
                        register={register}
                        label="Email"
                        placeholder="john.doe@ruline.io"
                        error={errors.email?.message}
                    />
                </div>
            </AuthForm>
        </div>
    );
}
