import { useSettings } from '@hooks/settings';
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
    oauth: {
        google: {
            text: string;
        };
    };
    magicLink: {
        email: string;
        form: React.ReactNode;
        sent: boolean;
        onSubmit: React.FormEventHandler<HTMLFormElement> | undefined;
        button: {
            text: string;
            disabled: boolean;
        };
    };
    error?: string;
}

export function AuthForm(props: AuthFormProps) {
    const { title, subtitle, oauth, magicLink, error } = props;
    const { settings } = useSettings();
    const oauthEnabled = settings?.google_auth_enabled;
    const containerClass =
        'flex flex-col justify-center bg-surface text-surface-text p-6 md:p-8 rounded-lg shadow-md w-11/12 md:w-1/2 lg:w-1/3 xl:w-1/4';

    if (magicLink.sent) {
        return (
            <div className={containerClass}>
                <div className="space-y-6 text-center">
                    <h1 className="text-2xl font-bold mb-4">
                        Check your inbox
                    </h1>
                    <p className="text-primary-text text-center px-4">
                        Use the link we sent to{' '}
                        <span className="font-bold text-accent">
                            {magicLink.email}
                        </span>{' '}
                        to continue!
                    </p>
                    <p className="text-primary-text text-opacity-60 text-center px-4 text-xs">
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
            {settings?.google_auth_enabled && (
                <GoogleButton text={oauth.google.text} />
            )}
            {oauthEnabled && settings?.magic_link_enabled && (
                <div className="flex items-center justify-center w-full mt-6 mb-1 opacity-70">
                    <div className="border-t border-surface-text w-1/2" />
                    <p className="mx-2 text-primary-text text-sm">or</p>
                    <div className="border-t border-surface-text w-1/2" />
                </div>
            )}
            {settings?.magic_link_enabled && magicLink.form && (
                <div className="flex flex-col items-center w-full">
                    <form
                        onSubmit={magicLink.onSubmit}
                        className="w-full max-w-sm"
                    >
                        {magicLink.form}
                        <div className="mt-6">
                            <Button
                                disabled={magicLink.button.disabled}
                                type="submit"
                                text={magicLink.button.text}
                                size="small"
                            />
                        </div>
                    </form>
                </div>
            )}
            {error && (
                <div className="mt-6 text-center">
                    <Alert message={error} type="error" />
                </div>
            )}
        </div>
    );
}
