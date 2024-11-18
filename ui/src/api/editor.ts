import * as v from 'valibot';

export async function fetchEditorState(
    projectId: string,
    workflowId: string,
    version: number,
): Promise<EditorState> {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}/versions/${version}/editor`,
    );
    if (response.status !== 200) {
        throw new Error(`Error fetching editor state: ${response.statusText}`);
    }
    const data = await response.json();
    return v.parse(EditorState, data);
}

export const EditorState = v.object({
    members: v.array(
        v.object({
            id: v.string(),
            name: v.string(),
            avatar: v.string(),
        }),
    ),
});

export type EditorState = v.InferInput<typeof EditorState>;

export const EditorEventMemberJoined = v.object({
    event: v.literal('member_joined'),
    member_id: v.string(),
    name: v.string(),
    avatar: v.string(),
});

export type EditorEventMemberJoined = v.InferInput<
    typeof EditorEventMemberJoined
>;

export const EditorEventMemberLeft = v.object({
    event: v.literal('member_left'),
    member_id: v.string(),
});

export type EditorEventMemberLeft = v.InferInput<typeof EditorEventMemberLeft>;

export const EditorEvent = v.variant('event', [
    EditorEventMemberJoined,
    EditorEventMemberLeft,
]);

export type EditorEvent = v.InferInput<typeof EditorEvent>;
