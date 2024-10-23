import type colors from './props/color';
import type { sizes } from './props/size';
import cn from './utils/cn';

const variants = ['normal', 'pill'] as const;
const types = ['filled', 'outlined'] as const;

export interface BadgeProps {
    label: string;
    color?: (typeof colors)[number];
    size?: (typeof sizes)[number];
    variant?: (typeof variants)[number];
    type?: (typeof types)[number];
    className?: string;
}

export function Badge(props: BadgeProps) {
    const {
        label,
        color = 'teal',
        size = 'xs',
        variant = 'normal',
        type = 'filled',
        className,
    } = props;

    const classicClass = cn({
        'ring-teal-7 bg-teal-3 text-teal-11': color === 'teal',
        'ring-blue-7 bg-blue-3 text-blue-11': color === 'blue',
        'ring-red-7 bg-red-3 text-red-11': color === 'red',
        'ring-green-7 bg-green-3 text-green-11': color === 'green',
        'ring-amber-7 bg-amber-3 text-amber-11': color === 'amber',
        'ring-gray-7 bg-gray-3 text-gray-11': color === 'gray',
        'ring-white/50 bg-white bg-opacity-15 text-white': color === 'white',
    });
    const outlinedClass = cn({
        'ring-teal-7 text-teal-11': color === 'teal',
        'ring-blue-7 text-blue-11': color === 'blue',
        'ring-red-7 text-red-11': color === 'red',
        'ring-green-7 text-green-11': color === 'green',
        'ring-amber-7 text-amber-11': color === 'amber',
        'ring-gray-7 text-gray-11': color === 'gray',
        'ring-white/50 text-white': color === 'white',
    });
    const sizeClass = cn({
        'text-xs': size === 'xs',
        'text-sm': size === 'sm',
        'text-md': size === 'md',
        'text-lg': size === 'lg',
    });

    const badgeClass = cn(
        'inline-flex items-center justify-center py-1',
        'px-3 ring-1 ring-inset truncate',
        'select-none',
        {
            'rounded-full': variant === 'pill',
            'rounded-md': variant === 'normal',
        },
        type === 'filled' ? classicClass : outlinedClass,
        sizeClass,
        className,
    );

    return <span className={badgeClass}>{label}</span>;
}
