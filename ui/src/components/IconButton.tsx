import React from 'react';
import { Button } from './Button';
import { Icon, type IconProps } from './Icon';
import type colors from './props/color';
import type { sizes } from './props/size';
import cn from './utils/cn';

const as = ['button', 'submit', 'reset'] as const;
const variants = ['classic', 'outline', 'soft', 'text'] as const;
const shapes = ['rounded', 'circle'] as const;

export interface IconButtonProps {
    color?: (typeof colors)[number];
    size?: (typeof sizes)[number];
    as?: (typeof as)[number];
    variant?: (typeof variants)[number];
    shape?: (typeof shapes)[number];
    icon: IconProps;
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
}

function IconButtonComponent(
    props: IconButtonProps,
    ref: React.Ref<HTMLButtonElement>,
) {
    const {
        color = 'teal',
        as = 'button',
        variant = 'classic',
        shape = 'rounded',
        icon,
        onClick,
        className,
    } = props;

    const btnClass = cn(
        {
            'rounded-full': shape === 'circle',
        },
        className,
    );

    return (
        <Button
            ref={ref}
            as={as}
            variant={variant}
            color={color}
            onClick={onClick}
            disabled={props.disabled}
            className={btnClass}
        >
            <Icon {...icon} />
        </Button>
    );
}

export const IconButton = React.forwardRef(IconButtonComponent);
