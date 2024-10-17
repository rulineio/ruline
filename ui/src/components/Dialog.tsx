import clsx from 'clsx';
import { Button, type ButtonProps } from './Button';
import { Icon, type IconType } from './Icon';

export interface DialogProps {
    open: boolean;
    onClose: () => void;
    variant?: 'form' | 'popup' | 'standard';
    title?: string;
    content?: string;
    icon?: IconType;
    buttons?: ButtonProps[];
    form?: React.ReactNode;
}

export function Dialog(props: DialogProps) {
    const dialogContainerClass = clsx('relative z-10', {
        hidden: !props.open,
    });
    const backdropClass = clsx(
        'inset-0 fixed z-10',
        'bg-dark bg-opacity-10 backdrop-filter backdrop-blur-sm',
        'transition-opacity duration-300 ease-in-out',
    );
    const headerClass = clsx(
        'flex items-center justify-between p-4 border-b border-surface-container',
        {
            'border-b-0': props.variant === 'popup',
        },
    );
    const bodyClass = clsx({
        'p-4': props.variant !== 'form',
        'p-3 sm:p-8': props.variant === 'form',
    });
    const footerClass = clsx(
        'flex items-center justify-end p-4 border-t border-surface-container',
        {
            'border-t-0': props.variant === 'popup',
            hidden: !props.buttons || props.variant === 'form',
        },
    );

    return (
        <div className={dialogContainerClass}>
            <div className={backdropClass} tabIndex={-1} aria-hidden="true" />
            <div
                className="fixed inset-0 z-10 w-screen overflow-y-auto"
                onClick={props.onClose}
                onKeyUp={(e) => {
                    if (e.key === 'Escape') {
                        props.onClose();
                    }
                }}
            >
                <div className="flex items-center text-center justify-center min-h-screen">
                    <div
                        className="relative transform rounded-lg bg-surface text-left"
                        onClick={(e) => e.stopPropagation()}
                        onKeyUp={(e) => e.stopPropagation()}
                    >
                        <div className={headerClass}>
                            <div className="flex items-center">
                                {props.icon && (
                                    <div className="mr-4">
                                        <Icon icon={props.icon} />
                                    </div>
                                )}
                                <div className="text-lg font-bold">
                                    {props.title}
                                </div>
                            </div>
                        </div>
                        <div className={bodyClass}>
                            {props.content && <div>{props.content}</div>}
                            {props.form && <div>{props.form}</div>}
                        </div>
                        <div className={footerClass}>
                            {props.buttons?.map((button) => (
                                <Button key={button.text} {...button} />
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
