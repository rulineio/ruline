import { Avatar, type AvatarProps } from './Avatar';
import { Badge, type BadgeProps } from './Badge';
import cn from './utils/cn';

export interface ListItem {
    id: string;
    title: string;
    subtitle?: string | React.ReactNode;
    avatar?: AvatarProps;
    badges?: BadgeProps[];
    actions?: { id: string; action: React.ReactNode }[];
    onClick?: () => void;
    className?: string;
}

export interface ListProps {
    items: ListItem[];
    className?: string;
    itemClassName?: string;
}

export function List(props: ListProps) {
    const { items, className } = props;

    if (!items.length) {
        return null;
    }

    const clickable = items.some((item) => item.onClick);
    const subtitleType = typeof items[0].subtitle;

    const listClass = cn('flex flex-col space-y-6 w-full', className);
    const itemClass = (itemClassName?: string) =>
        cn(
            'flex flex-row justify-between items-center',
            props.itemClassName,
            itemClassName,
        );
    const titleClass = cn('text-lg select-none', {
        'cursor-pointer': clickable,
    });
    const subtitleClass = cn('text-xs text-gray-11 max-w-sm sm:max-w-max');

    return (
        <ul className={listClass}>
            {items.map((item) => (
                <li key={item.id} className={itemClass(item.className)}>
                    <div className="flex items-center space-x-4">
                        {item.avatar && <Avatar {...item.avatar} />}
                        <div className="">
                            {clickable ? (
                                <button
                                    type="button"
                                    className={titleClass}
                                    onClick={item.onClick}
                                >
                                    {item.title}
                                </button>
                            ) : (
                                <div
                                    className={titleClass}
                                    onClick={item.onClick}
                                    onKeyDown={item.onClick}
                                >
                                    {item.title}
                                </div>
                            )}
                            {item.subtitle && subtitleType === 'string' && (
                                <span className={subtitleClass}>
                                    {item.subtitle}
                                </span>
                            )}
                            {item.subtitle && subtitleType !== 'string' && (
                                <div className={subtitleClass}>
                                    {item.subtitle}
                                </div>
                            )}
                        </div>
                    </div>
                    {(item.actions || item.badges) && (
                        <div className="space-x-4 flex flex-row items-center">
                            {item.badges && (
                                <div className="space-x-2 sm:flex flex-row items-center hidden">
                                    {item.badges.map((badge) => (
                                        <Badge key={badge.label} {...badge} />
                                    ))}
                                </div>
                            )}
                            {item.actions && (
                                <div className="space-x-2 flex flex-row items-center">
                                    {item.actions.map((action) => (
                                        <div key={`action_${action.id}`}>
                                            {action.action}
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    )}
                </li>
            ))}
        </ul>
    );
}
