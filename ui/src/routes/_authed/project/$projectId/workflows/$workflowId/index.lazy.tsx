import {
  updateWorkflow,
  type UpdateWorkflowForm,
  UpdateWorkflowSchema,
} from '@api/workflow'
import { Alert } from '@components/Alert'
import { Badge, type BadgeProps } from '@components/Badge'
import { Button } from '@components/Button'
import { Dialog } from '@components/Dialog'
import { Dropdown } from '@components/Dropdown'
import { Input } from '@components/Input'
import { List, type ListItem } from '@components/List'
import { Navbar } from '@components/Navbar'
import { Tooltip } from '@components/Tooltip'
import { valibotResolver } from '@hookform/resolvers/valibot'
import { useAuth } from '@hooks/auth'
import { useWorkflow, useWorkflowVersions } from '@hooks/workflow'
import { useQueryClient } from '@tanstack/react-query'
import { createLazyFileRoute, useNavigate } from '@tanstack/react-router'
import { useForm } from 'react-hook-form'
import { create } from 'zustand'

export const Route = createLazyFileRoute(
  '/_authed/project/$projectId/workflows/$workflowId/',
)({
  component: Workflow,
})

interface RenameWorkflowStore {
  isOpen: boolean
  open: () => void
  close: () => void
}
const useRenameWorkflowStore = create<RenameWorkflowStore>((set) => ({
  isOpen: false,
  open: () => set({ isOpen: true }),
  close: () => set({ isOpen: false }),
}))

interface ArchiveWorkflowStore {
  isOpen: boolean
  open: () => void
  close: () => void
}
const useArchiveWorkflowStore = create<ArchiveWorkflowStore>((set) => ({
  isOpen: false,
  open: () => set({ isOpen: true }),
  close: () => set({ isOpen: false }),
}))

function Workflow() {
  const { projectId, workflowId } = Route.useParams()
  const { workflow, isLoading } = useWorkflow(projectId, workflowId)
  const { isAdmin } = useAuth()
  const openRenameWorkflow = useRenameWorkflowStore((state) => state.open)
  const openArchiveWorkflow = useArchiveWorkflowStore((state) => state.open)
  const queryClient = useQueryClient()

  const activateWorkflow = async (workflowId: string) => {
    try {
      await updateWorkflow(projectId, workflowId, {
        status: 'active',
      })
      queryClient.invalidateQueries({
        queryKey: ['workflows', projectId, workflowId],
      })
    } catch (error) {
      if (error instanceof Error) {
        console.error(`Error activating workflow: ${error.message}`)
      }
    }
  }

  const navbar = (
    <Navbar
      previous={[
        {
          to: '/project/$projectId/workflows',
          params: { projectId },
          children: 'Workflows',
        },
      ]}
      title={workflow?.name ?? ''}
      projectId={projectId}
    />
  )

  if (isLoading || !workflow) {
    return <>{navbar}</>
  }

  const badge: BadgeProps = { label: 'Unknown', color: 'gray', size: 'sm' }
  if (!workflow.active_version) {
    badge.label = 'Draft'
  }
  if (workflow.active_version && workflow.status === 'active') {
    badge.color = 'teal'
    badge.label = 'Active'
  }
  if (workflow.status === 'archived') {
    badge.color = 'red'
    badge.label = 'Archived'
  }

  return (
    <>
      <RenameWorkflowDialog
        projectId={projectId}
        workflowId={workflowId}
        name={workflow.name}
      />
      <ArchiveWorkflowDialog projectId={projectId} workflowId={workflowId} />
      {navbar}
      <div className="p-8">
        <div className="space-y-4">
          <div className="flex flex-row justify-between sm:w-1/3 ">
            <div className="flex flex-row space-x-4 items-center">
              <h1 className="text-2xl font-bold select-none">
                {workflow.name}
              </h1>
              <Badge {...badge} />
            </div>
            <div>
              {isAdmin && (
                <Dropdown
                  align="start"
                  iconButton={{
                    color: 'gray',
                    className: 'p-2',
                    variant: 'text',
                    size: 'xs',
                    icon: {
                      icon: 'elipsis-vertical',
                      size: 4,
                    },
                  }}
                  items={[
                    {
                      id: 'rename',
                      label: 'Rename',
                      onClick: () => {
                        openRenameWorkflow()
                      },
                    },
                    {
                      separator: true,
                    },
                    workflow.status === 'active'
                      ? {
                          id: 'delete',
                          label: 'Archive',
                          color: 'red',
                          onClick: () => {
                            openArchiveWorkflow()
                          },
                        }
                      : {
                          id: 'activate',
                          label: 'Activate',
                          color: 'teal',
                          onClick: async () => {
                            await activateWorkflow(workflow.id)
                          },
                        },
                  ]}
                />
              )}
            </div>
          </div>
          <Tooltip
            side="right"
            trigger={
              <button
                type="button"
                className="mt-2 italic text-sm text-gray-11"
                onClick={() => {
                  navigator.clipboard.writeText(workflow.id)
                }}
              >
                {workflow.id}
              </button>
            }
          >
            Click to Copy
          </Tooltip>
        </div>
        <div className="mt-12 sm:w-1/3">
          <WorkflowVersions
            projectId={projectId}
            workflowId={workflowId}
            activeVersion={workflow.active_version}
          />
        </div>
      </div>
    </>
  )
}

function WorkflowVersions({
  projectId,
  workflowId,
  activeVersion,
}: {
  projectId: string
  workflowId: string
  activeVersion: number | null
}) {
  const { workflowVersions } = useWorkflowVersions(projectId, workflowId)
  const navigate = useNavigate()

  if (!workflowVersions) {
    return null
  }

  const items: ListItem[] = []
  for (const version of workflowVersions) {
    const item: ListItem = {
      id: version.version.toString(),
      title: `v${version.version}`,
      onClick: () => {
        navigate({
          to: '/project/$projectId/workflows/$workflowId/$version',
          params: {
            projectId,
            workflowId,
            version: version.version.toString(),
          },
        })
      },
      badges: [],
    }

    if (activeVersion !== version.version) {
      if (version.status === 'archived') {
        item.badges?.push({
          color: 'red',
          label: 'Archived',
        })
      }

      if (version.status === 'draft') {
        item.badges?.push({
          color: 'gray',
          label: 'Draft',
        })
      }

      if (version.status === 'in_review') {
        item.badges?.push({
          color: 'amber',
          label: 'In Review',
        })
      }

      if (version.status === 'published') {
        item.badges?.push({
          color: 'green',
          label: 'Published',
        })
      }
    }

    if (activeVersion === version.version) {
      item.badges?.push({
        color: 'teal',
        label: 'Active Version',
      })
    }

    items.push(item)
  }

  return (
    <div className="py-6 bg-gray-1 rounded-md border border-gray-7">
      <div className="pb-6 px-4 border-b border-gray-7">
        <h2 className="text-lg font-bold select-none text-gray-12">Versions</h2>
      </div>
      <List items={items} className="px-4 pt-4" />
    </div>
  )
}

function RenameWorkflowDialog({
  projectId,
  workflowId,
  name,
}: {
  projectId: string
  workflowId: string
  name: string
}) {
  const isOpen = useRenameWorkflowStore((state) => state.isOpen)
  const close = useRenameWorkflowStore((state) => state.close)
  const {
    register,
    handleSubmit,
    setError,
    reset,
    formState: { errors, isSubmitting, disabled },
  } = useForm<UpdateWorkflowForm>({
    resolver: valibotResolver(UpdateWorkflowSchema),
    values: { name },
  })

  const queryClient = useQueryClient()

  const submit = async (data: UpdateWorkflowForm) => {
    if (!workflowId) {
      return
    }
    try {
      await updateWorkflow(projectId, workflowId, data)
      await queryClient.invalidateQueries({
        queryKey: ['workflows', projectId, workflowId],
      })
      reset()
      close()
    } catch (error) {
      if (error instanceof Error) {
        setError('root', { message: error.message })
      }
    }
  }

  return (
    <Dialog
      title="Rename Workflow"
      description={`Rename the workflow "${name}"`}
      open={isOpen}
      onOpenChange={close}
    >
      <form onSubmit={handleSubmit(submit)}>
        <div className="grid grid-cols-2">
          <div className="col-span-2">
            <Input
              name="name"
              register={register}
              label="New Name"
              placeholder="My Awsome Workflow"
              error={errors.name?.message}
            />
          </div>
        </div>
        <Button
          as="submit"
          color="teal"
          variant="classic"
          className="mt-6"
          disabled={disabled || isSubmitting}
        >
          Rename
        </Button>
        {errors.root?.message && (
          <div className="mt-4">
            <Alert message={errors.root.message} type="error" />
          </div>
        )}
      </form>
    </Dialog>
  )
}

function ArchiveWorkflowDialog({
  projectId,
  workflowId,
}: {
  projectId: string
  workflowId: string
}) {
  const isOpen = useArchiveWorkflowStore((state) => state.isOpen)
  const close = useArchiveWorkflowStore((state) => state.close)

  const queryClient = useQueryClient()

  const submit = async () => {
    if (!workflowId) {
      return
    }
    try {
      await updateWorkflow(projectId, workflowId, {
        status: 'archived',
      })
      await queryClient.invalidateQueries({
        queryKey: ['workflows', projectId, workflowId],
      })
      close()
    } catch (error) {
      if (error instanceof Error) {
        console.error(error.message)
      }
    }
  }

  return (
    <Dialog
      variant="alert"
      title="Archive Workflow"
      description="If you archive this workflow, it will no longer be available for use. You can always activate it again later."
      open={isOpen}
      onOpenChange={close}
      action={{
        children: 'Archive',
        color: 'red',
        onClick: submit,
      }}
      cancel={{
        children: 'Cancel',
        color: 'gray',
        onClick: () => close(),
      }}
    />
  )
}
