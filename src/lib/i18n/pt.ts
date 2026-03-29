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

  // File status
  binary: "(binário)",
  head: "HEAD",
  stagedBadge: "S",

  // Detail pane
  filesChanged: (n: number) => `${n} arquivo${n !== 1 ? "s" : ""} alterado${n !== 1 ? "s" : ""}`,
  parents: "Pais",
};
