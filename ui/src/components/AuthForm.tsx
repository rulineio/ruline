import { Link } from '@tanstack/react-router';
import { Alert } from './Alert';
import { Button, GoogleButton } from './Button';

export interface AuthFormProps {
    title: string;
    subtitle: {
        text: string;
        link: {
            text: string;
            href: string;
        };
    };
    email: string;
    onSubmit: React.FormEventHandler<HTMLFormElement> | undefined;
    oauth: {
        google: {
            text: string;
            enabled: boolean;
        };
    };
    linkSent: boolean;
    children?: React.ReactNode;
    button: {
        text: string;
        disabled: boolean;
    };
    error?: string;
}

export function AuthForm(props: AuthFormProps) {
    const {
        title,
        subtitle,
        email,
        onSubmit,
        oauth,
        linkSent,
        children,
        button,
        error,
    } = props;

    const oauthEnabled = oauth.google.enabled;

    const containerClass =
        'flex flex-col enter justify-center bg-white p-6 md:p-8 rounded-lg shadow-md w-11/12 md:w-1/4';

    if (linkSent) {
        return (
            <div className={containerClass}>
                <div className="space-y-6 text-center">
                    <h1 className="text-2xl font-bold mb-4">
                        Check your inbox
                    </h1>
                    <p className="text-gray-500 text-center px-4">
                        Use the link we sent to{' '}
                        <span className="font-bold text-blue-500">{email}</span>{' '}
                        to continue!
                    </p>
                    <p className="text-gray-500 text-center px-4 text-xs">
                        If you don't see the email, check your spam folder or
                        contact us{' '}
                        <a
                            href="mailto:support@ruline.io"
                            className="font-bold"
                        >
                            here
                        </a>
                        .
                    </p>
                </div>
            </div>
        );
    }

    return (
        <div className={containerClass}>
            <div className="mb-8">
                <h1 className="text-2xl font-bold mb-4">{title}</h1>
                <h2 className="text-sm opacity-75">
                    {subtitle.text}{' '}
                    <span className="font-bold">
                        <Link to={subtitle.link.href}>
                            {subtitle.link.text}
                        </Link>
                    </span>
                </h2>
            </div>
            {oauth.google.enabled && <GoogleButton text={oauth.google.text} />}
            {oauthEnabled && (
                <div className="flex items-center justify-center w-full mt-6 mb-1 opacity-70">
                    <div className="border-t border-gray-300 w-1/2" />
                    <p className="mx-2 text-gray-500 text-sm">or</p>
                    <div className="border-t border-gray-300 w-1/2" />
                </div>
            )}
            <div className="flex flex-col items-center w-full">
                <form onSubmit={onSubmit} className="w-full max-w-sm">
                    {children}
                    <div className="mt-6">
                        <Button
                            disabled={button.disabled}
                            type="submit"
                            text={button.text}
                        />
                    </div>
                </form>
            </div>
            {error && (
                <div className="mt-6 text-center">
                    <Alert message={error} type="error" />
                </div>
            )}
        </div>
    );
}
