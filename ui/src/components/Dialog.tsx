import { Button, type ButtonProps } from './Button';
import * as AlertDialog from '@radix-ui/react-alert-dialog';
import * as RDialog from '@radix-ui/react-dialog';
import cn from './utils/cn';

const variants = ['classic', 'alert'] as const;

export interface DialogProps {
    button?: React.PropsWithChildren<ButtonProps>;
    title: string;
    description: string;
    variant?: (typeof variants)[number];
    action?: React.PropsWithChildren<ButtonProps>;
    cancel?: React.PropsWithChildren<ButtonProps>;
    onOpenChange?: (open: boolean) => void;
    open?: boolean;
}

export function Dialog(props: React.PropsWithChildren<DialogProps>) {
    const {
        button,
        title,
        description,
        variant = 'classic',
        action,
        children,
        open,
        onOpenChange,
    } = props;

    const triggerClass = cn({ hidden: variant === 'alert' });
    const overlayClass = cn('fixed inset-0 bg-teal-1 bg-opacity-50');
    const contentClass = cn(
        'fixed left-1/2 top-1/2 max-h-[85vh] w-[90vw] max-w-[450px]',
        '-translate-x-1/2 -translate-y-1/2 rounded-md',
        'bg-teal-2 p-8 focus:outline-none',
        {
            'space-x-4': variant === 'alert',
        },
    );
    const titleClass = cn('text-white text-lg font-semibold mb-4');
    const descriptionClass = cn('text-sm text-gray-11 mb-8');

    if (variant === 'alert') {
        const { cancel } = props;

        if (!action && !cancel) {
            throw new Error(
                'An alert dialog must have an action or a cancel button',
            );
        }

        return (
            <AlertDialog.Root open={open} onOpenChange={onOpenChange}>
                <AlertDialog.Trigger asChild className={triggerClass}>
                    <Button {...button} />
                </AlertDialog.Trigger>
                <AlertDialog.Portal>
                    <AlertDialog.Overlay className={overlayClass} />
                    <AlertDialog.Content className={contentClass}>
                        <AlertDialog.Title className={titleClass}>
                            {title}
                        </AlertDialog.Title>
                        <AlertDialog.Description className={descriptionClass}>
                            {description}
                        </AlertDialog.Description>
                        <AlertDialog.Cancel>
                            <Button {...cancel} />
                        </AlertDialog.Cancel>
                        <AlertDialog.Action>
                            <Button {...action} />
                        </AlertDialog.Action>
                    </AlertDialog.Content>
                </AlertDialog.Portal>
            </AlertDialog.Root>
        );
    }

    return (
        <RDialog.Root>
            <RDialog.Trigger asChild>
                <Button {...button} />
            </RDialog.Trigger>
            <RDialog.Overlay className={overlayClass} />
            <RDialog.Content className={contentClass}>
                <RDialog.Title className={titleClass}>{title}</RDialog.Title>
                <RDialog.Description className={descriptionClass}>
                    {description}
                </RDialog.Description>
                {children}
                <RDialog.Close className={cn({ hidden: !action })}>
                    <Button {...action} />
                </RDialog.Close>
            </RDialog.Content>
        </RDialog.Root>
    );
}
