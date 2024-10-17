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
}

export function List(props: ListProps) {
    const { items } = props;

    return (
        <ul className="flex flex-col space-y-6 w-full">
            {items.map((item) => (
                <li key={item.id} className="flex flex-row justify-between">
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
                        <div className="mt-2 flex space-x-2">
                            {item.badges.map((badge) => (
                                <Badge key={badge.label} {...badge} />
                            ))}
                        </div>
                    )}
                    {item.actions && (
                        <div className="mt-2 flex space-x-2">
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
