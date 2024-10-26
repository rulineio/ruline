import { Link, type LinkProps } from '@tanstack/react-router';
import { Button, type ButtonProps } from './Button';
import type colors from './props/color';
import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import { IconButton, type IconButtonProps } from './IconButton';
import cn from './utils/cn';

const sides = ['top', 'right', 'bottom', 'left'] as const;
const aligns = ['start', 'center', 'end'] as const;

export interface DropdownItemProps {
    color?: (typeof colors)[number];
    id?: string;
    label?: string;
    link?: LinkProps;
    onClick?: () => void;
    separator?: boolean;
    items?: DropdownItemProps[];
}

export interface DropdownProps {
    button?: React.PropsWithChildren<ButtonProps>;
    iconButton?: React.PropsWithChildren<IconButtonProps>;
    items: DropdownItemProps[];
    side?: (typeof sides)[number];
    align?: (typeof aligns)[number];
}

export function Dropdown(props: DropdownProps) {
    const { button, iconButton, items, align, side } = props;

    const contentClass = cn(
        'min-w-28 rounded-md will-change-[opacity,transform]',
        'bg-gray-1 text-white px-2 py-3 shadow-sm shadow-gray-7 border border-gray-7',
    );

    const itemClass = (item: DropdownItemProps) =>
        cn(
            'flex items-center justify-between px-4 py-2select-none leading-none outline-none',
            'text-sm py-2 px-3 rounded-md select-none',
            {
                'hover:bg-gray-4': item.color === 'gray' || !item.color,
                'text-red-11 hover:bg-red-4': item.color === 'red',
                'text-teal-11 hover:bg-teal-4': item.color === 'teal',
            },
        );

    const content: React.ReactNode[] = [];
    for (const item of items) {
        if (item.separator) {
            content.push(
                <DropdownMenu.Separator
                    key={content.length}
                    className="mx-1 my-3 h-px bg-gray-6"
                />,
            );
        } else {
            content.push(
                <DropdownMenu.Item
                    key={content.length}
                    onSelect={item.onClick}
                    className={itemClass(item)}
                >
                    {item.link ? (
                        <Link
                            className="hover:cursor-default w-full"
                            {...item.link}
                        >
                            {item.label}
                        </Link>
                    ) : (
                        item.label
                    )}
                </DropdownMenu.Item>,
            );
            if (item.items) {
                content.push(
                    <DropdownMenu.Sub key={content.length}>
                        <DropdownMenu.SubTrigger className={itemClass(item)}>
                            {item.label}
                        </DropdownMenu.SubTrigger>
                        <DropdownMenu.Portal>
                            {item.items.map((subItem) => (
                                <DropdownMenu.Item
                                    key={subItem.id ?? subItem.label}
                                    onSelect={subItem.onClick}
                                    className={itemClass(item)}
                                >
                                    {subItem.link ? (
                                        <Link
                                            className="hover:cursor-default"
                                            {...subItem.link}
                                        >
                                            {subItem.label}
                                        </Link>
                                    ) : (
                                        subItem.label
                                    )}
                                </DropdownMenu.Item>
                            ))}
                        </DropdownMenu.Portal>
                    </DropdownMenu.Sub>,
                );
            }
        }
    }

    let buttonComponent = null;
    if (button) {
        buttonComponent = <Button {...button} />;
    } else if (iconButton) {
        buttonComponent = <IconButton {...iconButton} />;
    }

    return (
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild>
                <div>{buttonComponent}</div>
            </DropdownMenu.Trigger>

            <DropdownMenu.Portal>
                <DropdownMenu.Content
                    className={contentClass}
                    sideOffset={5}
                    align={align}
                    side={side}
                >
                    {...content}
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu.Root>
    );
}
