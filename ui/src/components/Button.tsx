import React from 'react';
import type colors from './props/color';
import type { sizes } from './props/size';
import cn from './utils/cn';

const variants = ['classic', 'outline', 'soft', 'text'] as const;
const as = ['button', 'submit', 'reset'] as const;

export interface ButtonProps {
    color?: (typeof colors)[number];
    size?: (typeof sizes)[number];
    as?: (typeof as)[number];
    variant?: (typeof variants)[number];
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
}

function ButtonComponent(
    props: React.PropsWithChildren<ButtonProps>,
    ref: React.Ref<HTMLButtonElement>,
) {
    const {
        color = 'teal',
        size = 'sm',
        variant = 'classic',
        as = 'button',
        children,
        onClick,
        disabled,
        className,
    } = props;

    const btnColor = disabled ? 'gray' : color;

    const classicClass = cn('text-white', {
        'bg-teal-9 hover:bg-teal-10': btnColor === 'teal',
        'bg-blue-9 hover:bg-blue-10': btnColor === 'blue',
        'bg-red-9 hover:bg-red-10': btnColor === 'red',
        'bg-green-9 hover:bg-green-10': btnColor === 'green',
        'bg-amber-9 hover:bg-amber-10': btnColor === 'amber',
        'bg-gray-9 hover:bg-gray-10': btnColor === 'gray',
        'bg-white bg-opacity-50 hover:bg-opacity-70 text-black':
            btnColor === 'white',
    });
    const outlineClass = cn('text-teal outline outline-2', {
        'outline-teal-7 hover:outline-teal-8  text-teal-11':
            btnColor === 'teal',
        'outline-blue-7 hover:outline-blue-8 text-blue-11': btnColor === 'blue',
        'outline-red-7 hover:outline-red-8 text-red-11': btnColor === 'red',
        'outline-green-7 hover:outline-green-8 text-green-11':
            btnColor === 'green',
        'outline-amber-7 hover:outline-amber-8 text-amber-11':
            btnColor === 'amber',
        'outline-gray-7 hover:outline-gray-8 text-gray-11': btnColor === 'gray',
    });
    const softClass = cn('text-teal', {
        'bg-teal-3 hover:bg-teal-4 text-teal-11': btnColor === 'teal',
        'bg-blue-3 hover:bg-blue-4 text-blue-11': btnColor === 'blue',
        'bg-red-3 hover:bg-red-4 text-red-11': btnColor === 'red',
        'bg-green-3 hover:bg-green-4 text-green-11': btnColor === 'green',
        'bg-amber-3 hover:bg-amber-4 text-amber-11': btnColor === 'amber',
        'bg-gray-3 hover:bg-gray-4 text-gray-11': btnColor === 'gray',
        'bg-white/3 hover:bg-white/4 text-white/11': btnColor === 'white',
        'bg-black/3 hover:bg-black/4 text-black/11': btnColor === 'black',
    });
    const textClass = cn('text-teal', {
        'text-teal-11 hover:bg-teal-3': btnColor === 'teal',
        'text-blue-11 hover:bg-blue-3': btnColor === 'blue',
        'text-red-11 hover:bg-red-3': btnColor === 'red',
        'text-green-11 hover:bg-green-3': btnColor === 'green',
        'text-amber-11 hover:bg-amber-3': btnColor === 'amber',
        'text-gray-11 hover:bg-gray-3': btnColor === 'gray',
    });
    const sizeClass = cn({
        'text-xs': size === 'xs',
        'text-sm': size === 'sm',
        'text-base': size === 'md',
        'text-lg': size === 'lg',
    });

    const btnClass = cn(
        'p-3 rounded-md cursor-pointer disabled:cursor-not-allowed',
        'w-full flex items-center justify-center focus:outline-none',
        sizeClass,
        variant === 'classic' && classicClass,
        variant === 'outline' && outlineClass,
        variant === 'soft' && softClass,
        variant === 'text' && textClass,
        className,
    );

    return (
        <button
            ref={ref}
            disabled={disabled}
            type={as}
            className={btnClass}
            onClick={onClick}
            aria-disabled={disabled}
        >
            <span className="contents select-none" aria-hidden>
                {children}
            </span>
        </button>
    );
}

export function GoogleButton(props: React.PropsWithChildren<ButtonProps>) {
    const { children } = props;
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
                    {children}
                </div>
            </button>
        </form>
    );
}

export const Button = React.forwardRef(ButtonComponent);
