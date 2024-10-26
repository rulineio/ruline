import * as RPopover from '@radix-ui/react-popover';
import { Button, type ButtonProps } from './Button';
import cn from './utils/cn';

const sides = ['top', 'right', 'bottom', 'left'] as const;
const aligns = ['start', 'center', 'end'] as const;

export interface PopoverProps {
    button: React.PropsWithChildren<ButtonProps>;
    side?: (typeof sides)[number];
    align?: (typeof aligns)[number];
    className?: string;
}

export function Popover(props: React.PropsWithChildren<PopoverProps>) {
    const { button, side, align, className, children } = props;

    const contentClass = cn(
        'bg-gray-2 rounded-md p-2 shadow-lg text-white w-40',
        'outline outline-1 outline-gray-7 z-30',
        className,
    );

    return (
        <RPopover.Root>
            <RPopover.Trigger asChild>
                <Button {...button}>{button.children}</Button>
            </RPopover.Trigger>
            <RPopover.Portal>
                <RPopover.Content
                    side={side}
                    align={align}
                    className={contentClass}
                    sideOffset={5}
                >
                    {children}
                </RPopover.Content>
            </RPopover.Portal>
        </RPopover.Root>
    );
}
