import * as R from 'remeda';
import clsx from 'clsx';
import { useEffect, useState } from 'react';

export interface SelectProps {
    value: string;
    onChange: (value: string) => void;
    options: { value: string; label: string }[];
    label: string;
    className?: string;
}

export function Select(props: SelectProps) {
    const { value, onChange, options, label, className } = props;
    const [open, setOpen] = useState(false);
    const [preSelectedValue, setPreSelectedValue] = useState(value);

    useEffect(() => {
        setPreSelectedValue(value);
    }, [value]);

    const listClass = clsx(
        'absolute z-10 mt-2 max-h-56 w-full overflow-auto rounded-md',
        'sm:text-sm focus:outline-none shadow-sm shadow-accent',
        {
            hidden: !open,
        },
    );
    const overlayClass = clsx('fixed inset-0 z-10', {
        hidden: !open,
    });
    const containerClass = clsx('my-2', className);

    return (
        <div className={containerClass}>
            <label htmlFor={label} className="text-xs">
                {label}
            </label>
            <div
                className={overlayClass}
                onClick={() => setOpen(false)}
                onKeyUp={() => setOpen(false)}
            />
            <div className="relative mt-2 z-30">
                <button
                    type="button"
                    className="relative w-full mt-2 block pr-10 py-2 shadow-sm rounded-md focus:outline-none sm:text-sm bg-background text-primary-text focus:ring-2 focus:ring-accent"
                    aria-haspopup="listbox"
                    aria-expanded="true"
                    aria-labelledby="listbox-label"
                    id={label}
                    onClick={() => setOpen(!open)}
                    onKeyDown={(e) => {
                        if (
                            !open &&
                            R.isIncludedIn(e.key, ['ArrowDown', 'ArrowUp'])
                        ) {
                            return setOpen(true);
                        }

                        if (e.key === 'Enter' && open) {
                            onChange(preSelectedValue);
                        }

                        if (e.key === 'Escape') {
                            return setOpen(false);
                        }

                        const index = options.findIndex(
                            (option) => option.value === preSelectedValue,
                        );
                        if (e.key === 'ArrowDown') {
                            if (index === options.length - 1) {
                                return setPreSelectedValue(options[0].value);
                            }
                            return setPreSelectedValue(
                                options[index + 1].value,
                            );
                        }
                        if (e.key === 'ArrowUp') {
                            if (index === 0) {
                                return setPreSelectedValue(
                                    options[options.length - 1].value,
                                );
                            }
                            return setPreSelectedValue(
                                options[index - 1].value,
                            );
                        }
                    }}
                >
                    <span className="flex items-center">
                        <span className="ml-3 block truncate">{value}</span>
                    </span>
                    <span className="pointer-events-none absolute inset-y-0 right-0 ml-3 flex items-center pr-2">
                        <svg
                            className="h-5 w-5"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                            data-slot="icon"
                        >
                            <path
                                fillRule="evenodd"
                                d="M10.53 3.47a.75.75 0 0 0-1.06 0L6.22 6.72a.75.75 0 0 0 1.06 1.06L10 5.06l2.72 2.72a.75.75 0 1 0 1.06-1.06l-3.25-3.25Zm-4.31 9.81 3.25 3.25a.75.75 0 0 0 1.06 0l3.25-3.25a.75.75 0 1 0-1.06-1.06L10 14.94l-2.72-2.72a.75.75 0 0 0-1.06 1.06Z"
                                clipRule="evenodd"
                            />
                        </svg>
                    </span>
                </button>
                <ul className={listClass} tabIndex={-1}>
                    {options.map((option) => (
                        <li
                            key={option.value}
                            className={clsx(
                                'relative cursor-pointer select-none py-2 pl-3 pr-9',
                                'hover:bg-accent/50 hover:text-accent-text',
                                {
                                    'bg-background text-surface-text':
                                        preSelectedValue !== option.value,
                                    'bg-accent text-accent-text':
                                        preSelectedValue === option.value,
                                },
                            )}
                            onClick={() => {
                                onChange(option.value);
                                setOpen(false);
                            }}
                            onKeyUp={() => {
                                onChange(option.value);
                                setOpen(false);
                            }}
                        >
                            <div className="flex items-center">
                                <span className="block truncate font-normal">
                                    {option.label}
                                </span>
                            </div>
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    );
}
