import { type Scene, lengthToSeconds, realLength } from '$lib/scenedatabase';

export class AtomicSceneFilter {
  constructor(private negate: boolean = false) {}

  public matches(s: Scene): boolean {
    return this.test(s) != this.negate;
  }

  // this will be overwritten with the actual function
  // by parse
  private test(s: Scene): boolean {
    return true; // an empty filter is true for all scenes
  }

  public parse(s: string): string {
    this.test = (s) => true; // reset filter
    this.negate = false;

    s = s.trimStart();

    if (!s || s[0] === ')') return s;

    if (s[0] === '!') {
      this.negate = true;
      s = s.slice(1).trimStart();
    }

    // Figure out the end of the first word
    let end = 0;
    while (end < s.length && s[0] !== ' ' && s[0] !== '(') {
      end++;
    }

    const w = s.slice(0, end);
    s = s.slice(end);

    // I apologize for the following ugly and probably buggy block of code.
    // It takes a single filter condition (for example "numgirls>=2")
    // and creates a function that does evaluate exactly that condition,
    // for example:
    //            test.eval = (s) => s.num_girls >= 2
    // which is very simple and supposedly makes the evaluation efficient.
    // Much of this could be generalized. However, I am not sure how to
    // do this without losing efficiency.
    if (/[:<>=]/.test(w)) {
      if (w.startsWith('actor:')) this.test = (s) => s.actors?.indexOf(w.slice(6)) !== -1;
      else if (w.startsWith('website:')) this.test = (s) => s.website?.indexOf(w.slice(8)) !== -1;
      else if (w.startsWith('file:'))
        this.test = (s) =>
          s.file_name.indexOf(w.slice(5)) !== -1 || s.directory.indexOf(w.slice(5)) !== -1;
      else if (w.startsWith('numgirls')) {
        if (w.length > 8 && w[8] === '>')
          if (w.length > 9 && w[9] === '=') this.test = (s) => s.num_girls >= +w.slice(10);
          else this.test = (s) => s.num_girls > +w.slice(9);
        else if (w.length > 8 && w[8] === '<')
          if (w.length > 9 && w[9] === '=') this.test = (s) => s.num_girls <= +w.slice(10);
          else this.test = (s) => s.num_girls < +w.slice(9);
        else if (w.length > 8 && (w[8] === '=' || w[8] === ':'))
          this.test = (s) => s.num_girls == +w.slice(9);
      } else if (w.startsWith('numboys')) {
        if (w.length > 7 && w[7] === '>')
          if (w.length > 8 && w[8] === '=') this.test = (s) => s.num_boys >= +w.slice(9);
          else this.test = (s) => s.num_boys > +w.slice(8);
        else if (w.length > 7 && w[7] === '<')
          if (w.length > 8 && w[8] === '=') this.test = (s) => s.num_boys <= +w.slice(9);
          else this.test = (s) => s.num_boys < +w.slice(8);
        else if (w.length > 7 && (w[7] === '=' || w[7] === ':'))
          this.test = (s) => s.num_boys == +w.slice(8);
      } else if (w.startsWith('length')) {
        if (w.length > 6 && w[6] === '>')
          if (w.length > 7 && w[7] === '=')
            this.test = (s) => realLength(s) >= lengthToSeconds(w.slice(8));
          else this.test = (s) => realLength(s) > lengthToSeconds(w.slice(7));
        else if (w.length > 6 && w[6] === '<')
          if (w.length > 7 && w[7] === '=')
            this.test = (s) => realLength(s) <= lengthToSeconds(w.slice(8));
          else this.test = (s) => realLength(s) < lengthToSeconds(w.slice(7));
        else if (w.length > 6 && (w[6] === '=' || w[6] === ':'))
          this.test = (s) => realLength(s) == lengthToSeconds(w.slice(7));
      } else if (w.startsWith('year')) {
        if (w.length > 4 && w[4] === '>')
          if (w.length > 5 && w[5] === '=') this.test = (s) => !!s.year && s.year >= +w.slice(6);
          else this.test = (s) => !!s.year && s.year > +w.slice(5);
        else if (w.length > 4 && w[4] === '<')
          if (w.length > 5 && w[5] === '=') this.test = (s) => !!s.year && s.year <= +w.slice(6);
          else this.test = (s) => !!s.year && s.year < +w.slice(5);
        else if (w.length > 4 && (w[4] === '=' || w[4] === ':'))
          this.test = (s) => !!s.year && s.year == +w.slice(5);
      } else if (w.startsWith('score')) {
        if (w.length > 5 && w[5] === '>')
          if (w.length > 6 && w[6] === '=') this.test = (s) => s.score >= +w.slice(7);
          else this.test = (s) => s.score > +w.slice(6);
        else if (w.length > 5 && w[5] === '<')
          if (w.length > 6 && w[6] === '=') this.test = (s) => s.score <= +w.slice(7);
          else this.test = (s) => s.score < +w.slice(6);
        else if (w.length > 5 && (w[5] === '=' || w[5] === ':'))
          this.test = (s) => s.score == +w.slice(6);
      }
      return s;
    }
  }
}
