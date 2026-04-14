export default {
  // App
  appName: "GitFerrum",
  appSubtitle: "Cliente Git de alto desempenho",

  // Actions
  openRepository: "Abrir repositório",
  cloneRepository: "Clonar repositório",
  cancel: "Cancelar",
  commit: "Confirmar",
  create: "Criar",
  fetch: "Buscar",
  pull: "Puxar",
  push: "Enviar",
  close: "Fechar",
  browse: "Explorar",
  selectAll: "Selecionar tudo",
  deselectAll: "Desmarcar tudo",
  unstageAll: "Remover tudo do stage",

  // Loading states
  opening: "Abrindo...",
  cloning: "Clonando...",
  committing: "Confirmando...",
  loadingCommits: "Carregando commits...",
  loadingMore: "Carregando mais...",
  loading: "Carregando...",

  // Sections
  repositories: "Repositórios",
  branches: "Branches",
  remotes: "Remotos",
  tags: "Tags",
  stashes: "Stashes",
  commits: "Commits",
  details: "Detalhes",
  changes: "Alterações",
  staged: "Preparados",
  modified: "Modificados",
  untracked: "Não rastreados",

  // Empty states
  noChanges: "Sem alterações",
  noBranches: "Sem branches",
  noRemotes: "Sem remotos",
  noTags: "Sem tags",
  noStashes: "Sem stashes",
  noReposOpen: "Nenhum repositório aberto",
  noCommitsFound: "Nenhum commit encontrado",

  // Commit dialog
  commitChanges: "Confirmar alterações",
  commitMessage: "Mensagem do commit...",
  files: "Arquivos",

  // Clone dialog
  repositoryUrl: "URL do repositório",
  urlPlaceholder: "https://github.com/usuario/repo.git",
  cloneTo: "Clonar em",
  destPlaceholder: "/caminho/para/destino",

  // Changes panel
  stageNFiles: (n: number) => `Preparar ${n} arquivo${n !== 1 ? "s" : ""}`,
  noStagedChanges: "Sem alterações preparadas para confirmar",
  commitCreated: "Commit criado",
  nChanges: (n: number) => `${n} alteraç${n !== 1 ? "ões" : "ão"}`,

  // Context menu - remotes
  createLocalBranch: "Criar branch local",
  createCustomName: "Criar com nome personalizado...",
  browseDetached: "Explorar (desconectado)",
  newBranchName: "Nome da nova branch",

  // Tooltips
  openRepoTooltip: "Abrir repositório",
  cloneRepoTooltip: "Clonar repositório",
  closeRepoTooltip: "Fechar repositório",
  fetchTooltip: "Buscar",
  pullTooltip: (n: number) => `Puxar — ${n} commit${n !== 1 ? "s" : ""} atrás`,
  pushTooltip: (n: number) => `Enviar — ${n} commit${n !== 1 ? "s" : ""} à frente`,

  // Toasts
  repoOpened: (name: string) => `Aberto ${name}`,
  repoCloned: (name: string) => `Clonado ${name}`,
  fetchedFromRemote: "Buscado do remoto",
  pulledFromRemote: "Puxado do remoto",
  pushedToRemote: "Enviado ao remoto",
  pushNewBranchTitle: "Nova branch remota",
  pushNewBranchMessage: (branch: string, remote: string) =>
    `A branch "${branch}" não existe em "${remote}". Será criada e enviada.`,
  pushNewBranchConfirm: "Criar e Enviar",
  cannotPushDetached: "Não é possível enviar a partir de HEAD desconectado",
  noRemoteConfigured: "Nenhum remoto configurado. Adicione um primeiro com:\ngit remote add origin <url>",
  pushTimeout: "A operação expirou. Certifique-se de que sua chave SSH esteja carregada:\nssh-add ~/.ssh/sua_chave",
  pushRejected: "Push rejeitado: o remoto tem commits novos. Faça Pull primeiro para mesclar.",
  mergeConflictsTitle: "Conflitos de Merge",
  mergeConflictsMessage: (n: number) => `${n} arquivo${n !== 1 ? "s têm" : " tem"} conflitos de merge. Resolva manualmente, depois adicione e confirme.`,
  abortMerge: "Abortar Merge",
  resolveManually: "Resolver Manualmente",
  conflictFiles: "Arquivos com conflitos",
  repoStateMerging: "MERGING",
  repoStateRebasing: "REBASING",
  repoStateCherryPick: "CHERRY-PICKING",
  repoStateReverting: "REVERTING",
  checkedOut: (name: string) => `Mudado para ${name}`,
  branchCreated: (name: string) => `Branch ${name} criada`,
  browsingDetached: (ref: string) => `Explorando ${ref} (HEAD desconectado)`,

  // Relative time
  justNow: "agora",
  minutesAgo: (n: number) => `${n}m atrás`,
  hoursAgo: (n: number) => `${n}h atrás`,
  daysAgo: (n: number) => `${n}d atrás`,
  weeksAgo: (n: number) => `${n}sem atrás`,
  monthsAgo: (n: number) => `${n}mês atrás`,
  yearsAgo: (n: number) => `${n}a atrás`,

  // Updates
  updateAvailable: "Atualizacao Disponivel",
  updateMessage: (version: string) => `A versao ${version} esta disponivel. Deseja atualizar agora?`,
  updateNow: "Atualizar Agora",
  updating: "Atualizando...",
  updateLater: "Depois",
  upToDate: "Voce esta atualizado",
  checkingUpdates: "Verificando atualizacoes...",
  updateFailed: "Erro ao atualizar",
  updateRestart: "Atualizacao instalada. Reinicie para aplicar.",

  // Settings
  settings: "Configuracoes",
  language: "Idioma",
  settingsTooltip: "Configuracoes",
  general: "Geral",
  about: "Sobre",
  version: "Versao",
  checkForUpdates: "Verificar atualizacoes",
  gitConfiguration: "Configuração do Git",
  gitConfigDescription: "Configure sua identidade global do Git. Isso é usado para todos os repositórios que não têm configuração local.",
  gitUserName: "Nome",
  gitUserEmail: "E-mail",
  gitUserNamePlaceholder: "Seu nome",
  gitUserEmailPlaceholder: "seu@email.com",
  gitConfigSaved: "Configuração do Git salva",
  save: "Salvar",
  website: "Site",

  // File status
  binary: "(binário)",
  head: "HEAD",
  stagedBadge: "S",

  // Detail pane
  filesChanged: (n: number) => `${n} arquivo${n !== 1 ? "s" : ""} alterado${n !== 1 ? "s" : ""}`,
  parents: "Pais",

  // Branch operations
  deleteBranch: "Excluir branch",
  forceDeleteBranch: "Forçar exclusão da branch",
  renameBranch: "Renomear branch...",
  mergeBranchInto: "Mesclar na atual",
  branchDeleted: (name: string) => `Branch ${name} excluída`,
  branchRenamed: (oldName: string, newName: string) => `Renomeada ${oldName} para ${newName}`,
  cannotDeleteHead: "Não é possível excluir a branch atual",
  newBranchNameLabel: "Novo nome da branch",

  // Remote branch operations
  deleteRemoteBranch: "Excluir branch remota",
  deleteRemoteBranchConfirm: (name: string) => `Tem certeza que deseja excluir a branch remota "${name}"? Isso não pode ser desfeito.`,
  remoteBranchDeleted: (name: string) => `Branch remota ${name} excluída`,

  // Tag operations
  createTagLabel: "Criar tag...",
  deleteTagLabel: "Excluir tag",
  deleteRemoteTagLabel: "Excluir tag do remoto",
  tagName: "Nome da tag",
  tagMessage: "Mensagem (opcional, para tag anotada)",
  tagMessagePlaceholder: "Mensagem da tag...",
  tagCreated: (name: string) => `Tag ${name} criada`,
  tagDeleted: (name: string) => `Tag ${name} excluída`,
  remoteTagDeleted: (name: string) => `Tag remota ${name} excluída`,

  // Amend
  amendLastCommit: "Emendar último commit",
  amend: "Emendar",
  commitAmended: "Commit emendado",

  // Reset
  resetSoft: "Reset --soft até aqui",
  resetMixed: "Reset --mixed até aqui",
  resetHard: "Reset --hard até aqui",
  resetHardWarning: "Isso descartará todas as alterações no seu diretório de trabalho. Não pode ser desfeito.",
  resetComplete: (mode: string) => `Reset (${mode}) concluído`,

  // Stash operations
  applyStash: "Aplicar",
  dropStash: "Remover",
  stashApplied: (index: number) => `Aplicado stash@{${index}}`,
  stashDropped: (index: number) => `Removido stash@{${index}}`,
  confirmDropStash: "Tem certeza que deseja remover este stash? Não pode ser desfeito.",

  // Cherry-pick & Revert
  cherryPickCommit: "Cherry-pick",
  cherryPickApplied: "Cherry-pick aplicado",
  revertCommitLabel: "Reverter commit",
  commitReverted: "Commit revertido",

  // Copy
  copySha: "Copiar SHA",
  shaCopied: "SHA copiado para a área de transferência",

  // Merge
  mergeStarted: (name: string) => `Mesclado ${name}`,
  confirm: "Confirmar",
  deleteConfirm: "Excluir",

  // Merge dialog
  mergeBranches: "Mesclar Branches",
  mergeFrom: "Mesclar de",
  mergeInto: "Para (branch atual)",
  mergeAction: "Mesclar",
  merging: "Mesclando...",
  selectBranch: "Selecionar uma branch...",
  mergeDescription: (source: string, target: string) => `Isso mesclará "${source}" em "${target}". Se houver conflitos, você precisará resolvê-los manualmente.`,

  // Delete branch confirmation
  deleteBranchTitle: "Excluir Branch",
  deleteBranchConfirm: (name: string) => `Tem certeza que deseja excluir a branch local "${name}"?`,
  forceDeleteBranchTitle: "Forçar Exclusão de Branch",
  forceDeleteBranchConfirm: (name: string) => `Tem certeza que deseja forçar a exclusão da branch local "${name}"? Alterações não mescladas serão perdidas.`,

  // Discard changes
  discardChanges: "Descartar alterações",
  discardAllChanges: "Descartar todas as alterações",
  discardFileTitle: "Descartar Alterações",
  discardFileMessage: "Descartar todas as alterações neste arquivo? Esta ação não pode ser desfeita.",
  discardAllTitle: "Descartar Todas as Alterações",
  discardAllMessage: "Descartar todas as alterações não preparadas e remover todos os arquivos não rastreados? Esta ação não pode ser desfeita.",
  changesDiscarded: "Alterações descartadas",
  allChangesDiscarded: "Todas as alterações descartadas",
};
