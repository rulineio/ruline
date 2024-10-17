import clsx from 'clsx';
import { Icon, type IconType } from './Icon';

export interface ButtonProps {
    text: string;
    color?: 'primary' | 'secondary' | 'accent' | 'transparent' | 'error';
    size?: 'small' | 'medium' | 'large';
    type?: 'button' | 'submit' | 'reset';
    variant?: 'filled' | 'outlined';
    icon?: IconType;
    iconPosition?: 'left' | 'right';
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
}

export function Button(props: ButtonProps) {
    const {
        text,
        color = 'primary',
        size = 'medium',
        type = 'submit',
        variant: style = 'filled',
        iconPosition = 'left',
        icon,
        onClick,
        disabled,
        className,
    } = props;

    const buttonClass = clsx(
        'p-3 rounded-md hover:bg-opacity-80 disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed',
        className,
        {
            'w-full': className && !className.includes('w-'),
            'ring-1 ring-inset hover:ring-2': style === 'outlined',
            'bg-primary text-primary-text':
                color === 'primary' && style === 'filled',
            'border border-primary text-primary ring-primary':
                color === 'primary' && style === 'outlined',
            'bg-secondary text-secondary-text':
                color === 'secondary' && style === 'filled',
            'border border-secondary text-secondary ring-secondary':
                color === 'secondary' && style === 'outlined',
            'bg-accent text-accent-text':
                color === 'accent' && style === 'filled',
            'border border-accent text-accent ring-accent':
                color === 'accent' && style === 'outlined',
            'bg-error text-error-text': color === 'error' && style === 'filled',
            'border border-error text-error ring-error':
                color === 'error' && style === 'outlined',
            'bg-transparent': color === 'transparent',
            'text-xs': size === 'small',
            'text-sm': size === 'medium',
            'text-md': size === 'large',
            'flex flex-row items-center': !!icon,
        },
    );

    return (
        <button
            disabled={disabled}
            type={type}
            className={buttonClass}
            onClick={onClick}
        >
            {icon && iconPosition === 'left' && (
                <div className="mr-2">
                    <Icon icon={icon} size={4} />
                </div>
            )}
            <span>{text}</span>
            {icon && iconPosition === 'right' && (
                <div className="ml-2">
                    <Icon icon={icon} size={4} />
                </div>
            )}
        </button>
    );
}

export function GoogleButton(props: ButtonProps) {
    const { text } = props;

    return (
        <form action="/login/google" method="get" className="mt-4">
            <button
                type="submit"
                className="w-full border border-1 border-[#747775] text-sm text-[#1f1f1f] p-3 rounded-md bg-white"
            >
                <div className="flex items-center justify-center">
                    <svg
                        version="1.1"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 48 48"
                        className="w-5 h-5 mr-2"
                    >
                        <title>Google icon</title>
                        <path
                            fill="#EA4335"
                            d="M24 9.5c3.54 0 6.71 1.22 9.21 3.6l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.72 17.74 9.5 24 9.5z"
                        />
                        <path
                            fill="#4285F4"
                            d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 2.96-2.26 5.48-4.78 7.18l7.73 6c4.51-4.18 7.09-10.36 7.09-17.65z"
                        />
                        <path
                            fill="#FBBC05"
                            d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24c0 3.88.92 7.54 2.56 10.78l7.97-6.19z"
                        />
                        <path
                            fill="#34A853"
                            d="M24 48c6.48 0 11.93-2.13 15.89-5.81l-7.73-6c-2.15 1.45-4.92 2.3-8.16 2.3-6.26 0-11.57-4.22-13.47-9.91l-7.98 6.19C6.51 42.62 14.62 48 24 48z"
                        />
                        <path fill="none" d="M0 0h48v48H0z" />
                    </svg>
                    {text}
                </div>
            </button>
        </form>
    );
}
