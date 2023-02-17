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
