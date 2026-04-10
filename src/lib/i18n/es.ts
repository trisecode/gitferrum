export default {
  // App
  appName: "GitFerrum",
  appSubtitle: "Cliente Git de alto rendimiento",

  // Actions
  openRepository: "Abrir repositorio",
  cloneRepository: "Clonar repositorio",
  cancel: "Cancelar",
  commit: "Confirmar",
  create: "Crear",
  fetch: "Traer",
  pull: "Traer y combinar",
  push: "Enviar",
  close: "Cerrar",
  browse: "Explorar",
  selectAll: "Seleccionar todo",
  deselectAll: "Deseleccionar todo",
  unstageAll: "Quitar todo del stage",

  // Loading states
  opening: "Abriendo...",
  cloning: "Clonando...",
  committing: "Confirmando...",
  loadingCommits: "Cargando commits...",
  loadingMore: "Cargando más...",
  loading: "Cargando...",

  // Sections
  repositories: "Repositorios",
  branches: "Ramas",
  remotes: "Remotos",
  tags: "Etiquetas",
  stashes: "Stashes",
  commits: "Commits",
  details: "Detalles",
  changes: "Cambios",
  staged: "Preparados",
  modified: "Modificados",
  untracked: "Sin seguimiento",

  // Empty states
  noChanges: "Sin cambios",
  noBranches: "Sin ramas",
  noRemotes: "Sin remotos",
  noTags: "Sin etiquetas",
  noStashes: "Sin stashes",
  noReposOpen: "No hay repositorios abiertos",
  noCommitsFound: "No se encontraron commits",

  // Commit dialog
  commitChanges: "Confirmar cambios",
  commitMessage: "Mensaje del commit...",
  files: "Archivos",

  // Clone dialog
  repositoryUrl: "URL del repositorio",
  urlPlaceholder: "https://github.com/usuario/repo.git",
  cloneTo: "Clonar en",
  destPlaceholder: "/ruta/al/destino",

  // Changes panel
  stageNFiles: (n: number) => `Preparar ${n} archivo${n !== 1 ? "s" : ""}`,
  noStagedChanges: "No hay cambios preparados para confirmar",
  commitCreated: "Commit creado",
  nChanges: (n: number) => `${n} cambio${n !== 1 ? "s" : ""}`,

  // Context menu - remotes
  createLocalBranch: "Crear rama local",
  createCustomName: "Crear con nombre personalizado...",
  browseDetached: "Explorar (desconectado)",
  newBranchName: "Nombre de la nueva rama",

  // Tooltips
  openRepoTooltip: "Abrir repositorio",
  cloneRepoTooltip: "Clonar repositorio",
  closeRepoTooltip: "Cerrar repositorio",
  fetchTooltip: "Traer",
  pullTooltip: (n: number) => `Traer — ${n} commit${n !== 1 ? "s" : ""} atrás`,
  pushTooltip: (n: number) => `Enviar — ${n} commit${n !== 1 ? "s" : ""} adelante`,

  // Toasts
  repoOpened: (name: string) => `Abierto ${name}`,
  repoCloned: (name: string) => `Clonado ${name}`,
  fetchedFromRemote: "Cambios traídos del remoto",
  pulledFromRemote: "Cambios traídos y combinados",
  pushedToRemote: "Cambios enviados al remoto",
  pushNewBranchTitle: "Nueva rama remota",
  pushNewBranchMessage: (branch: string, remote: string) =>
    `La rama "${branch}" no existe en "${remote}". Se creará y se enviarán los cambios.`,
  pushNewBranchConfirm: "Crear y Enviar",
  cannotPushDetached: "No se puede enviar desde HEAD desconectado",
  noRemoteConfigured: "No hay remoto configurado. Agrega uno primero con:\ngit remote add origin <url>",
  pushTimeout: "La operación expiró. Asegúrate de que tu clave SSH esté cargada:\nssh-add ~/.ssh/tu_clave",
  pushRejected: "Push rechazado: el remoto tiene commits nuevos. Haz Pull primero para combinar.",
  mergeConflictsTitle: "Conflictos de Merge",
  mergeConflictsMessage: (n: number) => `${n} archivo${n !== 1 ? "s tienen" : " tiene"} conflictos de merge. Resuélvelos manualmente, luego agrega y confirma.`,
  abortMerge: "Abortar Merge",
  resolveManually: "Resolver Manualmente",
  conflictFiles: "Archivos con conflictos",
  repoStateMerging: "MERGING",
  repoStateRebasing: "REBASING",
  repoStateCherryPick: "CHERRY-PICKING",
  repoStateReverting: "REVERTING",
  checkedOut: (name: string) => `Cambiado a ${name}`,
  branchCreated: (name: string) => `Rama ${name} creada`,
  browsingDetached: (ref: string) => `Explorando ${ref} (HEAD desconectado)`,

  // Relative time
  justNow: "ahora",
  minutesAgo: (n: number) => `hace ${n}m`,
  hoursAgo: (n: number) => `hace ${n}h`,
  daysAgo: (n: number) => `hace ${n}d`,
  weeksAgo: (n: number) => `hace ${n}sem`,
  monthsAgo: (n: number) => `hace ${n}mes`,
  yearsAgo: (n: number) => `hace ${n}a`,

  // Updates
  updateAvailable: "Actualizacion Disponible",
  updateMessage: (version: string) => `La version ${version} esta disponible. Deseas actualizar ahora?`,
  updateNow: "Actualizar Ahora",
  updating: "Actualizando...",
  updateLater: "Despues",
  upToDate: "Estas al dia",
  checkingUpdates: "Buscando actualizaciones...",
  updateFailed: "Error al actualizar",
  updateRestart: "Actualizacion instalada. Reinicia para aplicar.",

  // Settings
  settings: "Configuracion",
  language: "Idioma",
  settingsTooltip: "Configuracion",
  general: "General",
  about: "Acerca de",
  version: "Version",
  checkForUpdates: "Buscar actualizaciones",

  // File status
  binary: "(binario)",
  head: "HEAD",
  stagedBadge: "S",

  // Detail pane
  filesChanged: (n: number) => `${n} archivo${n !== 1 ? "s" : ""} modificado${n !== 1 ? "s" : ""}`,
  parents: "Padres",

  // Branch operations
  deleteBranch: "Eliminar rama",
  forceDeleteBranch: "Forzar eliminación de rama",
  renameBranch: "Renombrar rama...",
  mergeBranchInto: "Combinar en la actual",
  branchDeleted: (name: string) => `Rama ${name} eliminada`,
  branchRenamed: (oldName: string, newName: string) => `Renombrada ${oldName} a ${newName}`,
  cannotDeleteHead: "No se puede eliminar la rama actual",
  newBranchNameLabel: "Nuevo nombre de rama",

  // Remote branch operations
  deleteRemoteBranch: "Eliminar rama remota",
  deleteRemoteBranchConfirm: (name: string) => `¿Estás seguro de eliminar la rama remota "${name}"? Esto no se puede deshacer.`,
  remoteBranchDeleted: (name: string) => `Rama remota ${name} eliminada`,

  // Tag operations
  createTagLabel: "Crear etiqueta...",
  deleteTagLabel: "Eliminar etiqueta",
  deleteRemoteTagLabel: "Eliminar etiqueta del remoto",
  tagName: "Nombre de la etiqueta",
  tagMessage: "Mensaje (opcional, para etiqueta anotada)",
  tagMessagePlaceholder: "Mensaje de la etiqueta...",
  tagCreated: (name: string) => `Etiqueta ${name} creada`,
  tagDeleted: (name: string) => `Etiqueta ${name} eliminada`,
  remoteTagDeleted: (name: string) => `Etiqueta remota ${name} eliminada`,

  // Amend
  amendLastCommit: "Enmendar último commit",
  amend: "Enmendar",
  commitAmended: "Commit enmendado",

  // Reset
  resetSoft: "Reset --soft hasta aquí",
  resetMixed: "Reset --mixed hasta aquí",
  resetHard: "Reset --hard hasta aquí",
  resetHardWarning: "Esto descartará todos los cambios en tu directorio de trabajo. No se puede deshacer.",
  resetComplete: (mode: string) => `Reset (${mode}) completado`,

  // Stash operations
  applyStash: "Aplicar",
  dropStash: "Eliminar",
  stashApplied: (index: number) => `Aplicado stash@{${index}}`,
  stashDropped: (index: number) => `Eliminado stash@{${index}}`,
  confirmDropStash: "¿Estás seguro de eliminar este stash? No se puede deshacer.",

  // Cherry-pick & Revert
  cherryPickCommit: "Cherry-pick",
  cherryPickApplied: "Cherry-pick aplicado",
  revertCommitLabel: "Revertir commit",
  commitReverted: "Commit revertido",

  // Copy
  copySha: "Copiar SHA",
  shaCopied: "SHA copiado al portapapeles",

  // Merge
  mergeStarted: (name: string) => `Combinado ${name}`,
  confirm: "Confirmar",
  deleteConfirm: "Eliminar",

  // Merge dialog
  mergeBranches: "Combinar Ramas",
  mergeFrom: "Combinar desde",
  mergeInto: "Hacia (rama actual)",
  mergeAction: "Combinar",
  merging: "Combinando...",
  selectBranch: "Seleccionar una rama...",
  mergeDescription: (source: string, target: string) => `Esto combinará "${source}" en "${target}". Si hay conflictos, deberás resolverlos manualmente.`,

  // Delete branch confirmation
  deleteBranchTitle: "Eliminar Rama",
  deleteBranchConfirm: (name: string) => `¿Estás seguro de que deseas eliminar la rama local "${name}"?`,
  forceDeleteBranchTitle: "Forzar Eliminación de Rama",
  forceDeleteBranchConfirm: (name: string) => `¿Estás seguro de que deseas forzar la eliminación de la rama local "${name}"? Los cambios no combinados se perderán.`,
};
