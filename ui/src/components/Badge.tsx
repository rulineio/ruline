import clsx from 'clsx';

export type BadgeColor =
    | 'primary'
    | 'secondary'
    | 'accent'
    | 'error'
    | 'warning'
    | 'success';

export interface BadgeProps {
    label: string;
    color?: BadgeColor;
    size?: 'small' | 'medium' | 'large';
    variant?: 'normal' | 'pill';
    type?: 'filled' | 'outlined';
}

export function Badge(props: BadgeProps) {
    const {
        label,
        color = 'primary',
        size = 'small',
        variant = 'normal',
        type = 'filled',
    } = props;

    const badgeClass = clsx(
        'inline-flex items-center py-1 px-3 ring-1 ring-inset',
        {
            'text-primary': color === 'primary',
            'text-secondary': color === 'secondary',
            'text-accent': color === 'accent',
            'text-error': color === 'error',
            'text-warning': color === 'warning',
            'text-success': color === 'success',
            'ring-primary/10 bg-primary/10':
                color === 'primary' && type === 'filled',
            'ring-secondary/10 bg-secondary/10':
                color === 'secondary' && type === 'filled',
            'ring-accent/10 bg-accent/10':
                color === 'accent' && type === 'filled',
            'ring-error/10 bg-error/10': color === 'error' && type === 'filled',
            'ring-warning/10 bg-warning/10':
                color === 'warning' && type === 'filled',
            'ring-success/10 bg-success/10':
                color === 'success' && type === 'filled',
            'ring-primary': color === 'primary' && type === 'outlined',
            'ring-secondary': color === 'secondary' && type === 'outlined',
            'ring-accent': color === 'accent' && type === 'outlined',
            'ring-error': color === 'error' && type === 'outlined',
            'ring-warning': color === 'warning' && type === 'outlined',
            'ring-success': color === 'success' && type === 'outlined',
            'rounded-full': variant === 'pill',
            'rounded-md': variant === 'normal',
            'text-xs': size === 'small',
            'text-sm': size === 'medium',
            'text-base': size === 'large',
        },
    );

    return <span className={badgeClass}>{label}</span>;
}
