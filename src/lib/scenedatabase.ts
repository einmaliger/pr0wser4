export interface Scene {
  file_name: string;
  name?: string;
  directory: string;
  thumb_file_name?: string;
  website?: string;
  actors?: string;
  cmd_parm?: string;
  tags: string[];
  begin?: number;
  end?: number;
  year?: number;
  length?: number;
  num_girls: number;
  num_boys: number;
  score: number;
}

export interface SceneDatabase {
  base_dir: string;
  film: Scene[];
}

export const EmptyScene: Scene = {
  file_name: '',
  directory: '',
  tags: [],
  num_girls: -1,
  num_boys: -1,
  score: 0
};

// Returns the given time to seconds
// The time may be given like 1:00:42
// or just a number of seconds
export function lengthToSeconds(length: string) {
  const components = length.split(':');
  let result = 0;
  for (const c of components) {
    result = result * 60 + +c;
  }
  return result;
}

export function realLength(s: Scene): number {
  if (!s.length && s.end) return s.end - (s.begin || 0);
  else return s.length || 0;
}
