import { login, type LoginForm, LoginSchema } from '@api/login';
import { AuthForm } from '@components/AuthForm';
import { Input } from '@components/Input';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useMagicLinkStore } from '@stores/magic-link';
import { createFileRoute, redirect } from '@tanstack/react-router';
import { useForm } from 'react-hook-form';
import { useShallow } from 'zustand/shallow';

export const Route = createFileRoute('/login')({
    preload: false,
    component: Login,
    beforeLoad: async ({ context }) => {
        const { refetch } = context.auth;
        const { data: session } = await refetch();
        console.log(session);
        if (session) {
            throw redirect({ to: '/' });
        }
    },
});

function Login() {
    const {
        register,
        handleSubmit,
        setError,
        formState: { errors, disabled, isSubmitting },
    } = useForm<LoginForm>({
        resolver: valibotResolver(LoginSchema),
    });

    const { sent, setSent, email, setEmail } = useMagicLinkStore(
        useShallow((state) => ({
            sent: state.sent,
            setSent: state.setSent,
            email: state.email,
            setEmail: state.setEmail,
        })),
    );

    const submit = async (data: LoginForm) => {
        try {
            await login(data);
            setSent();
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
        <div className="flex flex-col items-center justify-center min-h-screen">
            <AuthForm
                title="Login"
                subtitle={{
                    text: 'Don’t have an account?',
                    link: { text: 'Sign up', href: '/signup' },
                }}
                magicLink={{
                    onSubmit: handleSubmit(submit),
                    button: {
                        text: 'Sign In',
                        disabled: disabled || isSubmitting,
                    },
                    form: (
                        <div className="mb-6">
                            <Input
                                name="email"
                                register={register}
                                label="Email"
                                placeholder="john.doe@ruline.io"
                                error={errors.email?.message}
                            />
                        </div>
                    ),
                    sent,
                    email,
                }}
                oauth={{
                    google: {
                        text: 'Sign in with Google',
                    },
                }}
                error={errors.root?.message}
            />
        </div>
    );
}
