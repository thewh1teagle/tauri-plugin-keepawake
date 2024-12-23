import { invoke } from '@tauri-apps/api/core'

export interface KeepAwakeConfig {
  /** Keep display on */
  display: boolean;
  
  /** Keep system from idle sleeping */
  idle: boolean;
  
  /** Keep system from sleeping (Functionality and conditions for this to work vary by OS) */
  sleep: boolean;
}


export async function start(config?: KeepAwakeConfig): Promise<string | null> {
  return await invoke('plugin:keepawake|start', {config});
}

export async function stop(): Promise<string | null> {
  return await invoke('plugin:keepawake|stop');
}
