import clsx from 'clsx';
import { Avatar, type AvatarProps } from './Avatar';
import { Badge, type BadgeProps } from './Badge';

export interface ListItem {
    id: string;
    title: string;
    subtitle?: string;
    avatar?: AvatarProps;
    badges?: BadgeProps[];
    actions?: { id: string; action: React.ReactNode }[];
}

export interface ListProps {
    items: ListItem[];
    className?: string;
}

export function List(props: ListProps) {
    const { items, className } = props;

    const listClass = clsx('flex flex-col space-y-6 w-full', className);

    return (
        <ul className={listClass}>
            {items.map((item) => (
                <li
                    key={item.id}
                    className="flex flex-row justify-between items-center"
                >
                    <div className="flex items-center space-x-3">
                        {item.avatar && <Avatar {...item.avatar} />}
                        <div>
                            <div className="font-italic">{item.title}</div>
                            {item.subtitle && (
                                <div className="text-sm opacity-65">
                                    {item.subtitle}
                                </div>
                            )}
                        </div>
                    </div>
                    {item.badges && (
                        <div className="space-x-2 flex flex-row items-center">
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
                </li>
            ))}
        </ul>
    );
}
