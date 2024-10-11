import { valibotResolver } from '@hookform/resolvers/valibot';
import { createFileRoute, redirect } from '@tanstack/react-router';
import { useForm } from 'react-hook-form';
import { login, type LoginForm, LoginSchema } from '../api/login';
import { AuthForm } from '../components/AuthForm';
import { Input } from '../components/Input';
import { useLinkSentStore } from '../hooks/link';

export const Route = createFileRoute('/login')({
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
                magicLink={{
                    enabled: true,
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
                    linkSent,
                    email,
                }}
                oauth={{
                    google: {
                        enabled: true,
                        text: 'Sign in with Google',
                    },
                }}
                error={errors.root?.message}
            />
        </div>
    );
}
