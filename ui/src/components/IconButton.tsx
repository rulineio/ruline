import clsx from 'clsx';
import { Icon, type IconType } from './Icon';

export interface IconButtonProps {
    color?: 'primary' | 'secondary';
    size?: 'small' | 'medium' | 'large';
    type?: 'button' | 'submit' | 'reset';
    style?: 'filled' | 'outlined';
    shape?: 'rounded' | 'circle';
    icon: IconType;
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
}

export function IconButton(props: IconButtonProps) {
    const {
        color = 'primary',
        type = 'submit',
        style = 'filled',
        shape = 'rounded',
        icon,
        onClick,
        className,
    } = props;

    const buttonClass = clsx(
        'p-3 hover:bg-opacity-80 disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed',
        className,
        {
            'bg-primary text-primary-text':
                color === 'primary' && style === 'filled',
            'bg-secondary text-secondary-text':
                color === 'secondary' && style === 'filled',
            'border border-primary text-primary':
                color === 'primary' && style === 'outlined',
            'border border-secondary text-secondary':
                color === 'secondary' && style === 'outlined',
            'rounded-md': shape === 'rounded',
            'rounded-full': shape === 'circle',
        },
    );

    return (
        <button
            disabled={props.disabled}
            type={type}
            className={buttonClass}
            onClick={onClick}
        >
            <Icon icon={icon} />
        </button>
    );
}
