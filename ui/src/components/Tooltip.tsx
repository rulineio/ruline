import * as RTooltip from '@radix-ui/react-tooltip';
import cn from './utils/cn';

export interface TooltipProps {
    trigger: React.ReactElement;
    className?: string;
    delay?: number;
    side?: 'top' | 'right' | 'bottom' | 'left';
}

export function Tooltip(props: React.PropsWithChildren<TooltipProps>) {
    const { trigger, children, className, delay, side } = props;

    const contentClass = cn(
        'select-none rounded px-2 py-1.5 text-xs leading-none',
        'bg-gray-3 text-gray-12',
        'will-change-[transform,opacity]',
        className,
    );

    return (
        <RTooltip.Provider delayDuration={delay}>
            <RTooltip.Root>
                <RTooltip.Trigger asChild>{trigger}</RTooltip.Trigger>
                <RTooltip.Portal>
                    <RTooltip.Content
                        className={contentClass}
                        sideOffset={5}
                        side={side}
                    >
                        {children}
                        <RTooltip.Arrow className="fill-gray-3" />
                    </RTooltip.Content>
                </RTooltip.Portal>
            </RTooltip.Root>
        </RTooltip.Provider>
    );
}
