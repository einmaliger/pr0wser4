export default interface Scene {
  id: number;
  file_name: string;
  name?: string;
  directory: string;
  thumb_file_name?: string;
  website?: string;
  actors?: string;
  cmd_parm?: string;
  tags: string;
  begin?: number;
  end?: number;
  year?: number;
  length?: string;
  num_girls: number;
  num_boys: number;
  score: number;
}
