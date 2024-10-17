import clsx from 'clsx';
import { Button, type ButtonProps } from './Button';
import { useState } from 'react';

interface DropdowProps {
    title?: string;
    selectedOption: string;
    items: {
        label: string;
        onClick: () => void;
    }[];
    button?: Omit<
        ButtonProps,
        'text' | 'onClick' | 'type' | 'icon' | 'iconPostion'
    >;
    className?: string;
}

export function Dropdown(props: DropdowProps) {
    const {
        title,
        selectedOption,
        items,
        className,
        button = {
            color: 'transparent',
        },
    } = props;
    const [open, setOpen] = useState(false);

    const dropdownClass = clsx('relative inline-block text-left', className);
    const dropdownContentClass = clsx(
        'absolute right-0 z-40 mt-2 w-36 origin-top-right rounded-md bg-surface shadow-lg focus:outline-none',
        {
            hidden: !open,
        },
    );
    const dropdownOverlayClass = clsx('fixed inset-0 z-30', {
        hidden: !open,
    });

    return (
        <div className={dropdownClass}>
            <div>
                <Button
                    onClick={() => setOpen(!open)}
                    text={selectedOption}
                    type="button"
                    icon="chevron-down"
                    iconPosition="right"
                    {...button}
                />
            </div>
            <div
                className={dropdownContentClass}
                aria-orientation="vertical"
                aria-labelledby="menu-button"
                tabIndex={-1}
                role="menu"
            >
                {title && (
                    <>
                        <div className="py-1">
                            <p className="block px-2 py-1 text-sm">{title}</p>
                        </div>
                        <div className="border-t border-surface-container" />
                    </>
                )}
                <ul>
                    {items.map((item) => (
                        <li key={item.label}>
                            <Button
                                onClick={() => {
                                    item.onClick();
                                    setOpen(false);
                                }}
                                size="small"
                                color="transparent"
                                text={item.label}
                                className="font-bold text-left"
                            />
                        </li>
                    ))}
                </ul>
            </div>
            <div
                className={dropdownOverlayClass}
                onClick={() => {
                    setOpen(false);
                }}
                onKeyUp={(e) => {
                    if (e.key === 'Escape') {
                        setOpen(false);
                    }
                }}
            />
        </div>
    );
}

export default Dropdown;
