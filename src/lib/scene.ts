export default interface Scene {
  id: number;
  fileName: string;
  name?: string;
  directory: string;
  thumbFileName?: string;
  webSite?: string;
  actors?: string;
  cmdParm?: string;
  tags: string;
  begin?: number;
  end?: number;
  year?: number;
  length?: string;
  numGirls: number;
  numBoys: number;
  score: number;
}
