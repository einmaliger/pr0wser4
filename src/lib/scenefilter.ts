import { type Scene, lengthToSeconds, realLength } from '$lib/scenedatabase';
import { is_empty } from 'svelte/internal';

// Note: s is always left-trimmed at the beginning of parse functions

export class SceneFilter {
  private tree: OrNode;

  constructor() {
    this.tree = new OrNode();
  }

  public parse(s: string) {
    s.toLowerCase();
    this.tree = new OrNode(); // Let js simply drop the old one
    this.tree.parse(s.trimStart());
    // if this does not return an empty string, something went wrong
  }

  public matches(s: Scene): boolean {
    return this.tree.matches(s);
  }
}

class OrNode {
  private nodes: AndNode[];

  constructor(private readonly negate = false) {
    this.nodes = [];
  }

  public parse(s: string): string {
    // Parse an and node, which is a sequence of AtomicFilters and maybe other OrNodes (inside '()')
    // if a "or" follows, create a new andNode and continue
    // if not, we should be at the end of the string
    for (;;) {
      let n = new AndNode();
      s = n.parse(s).trimStart();
      this.nodes.push(n);

      if (s.startsWith('or ')) {
        s = s.slice(3);
        continue;
      }

      return s;
    }
  }

  public matches(s: Scene): boolean {
    for (const n of this.nodes) {
      if (n.matches(s)) {
        return !this.negate;
      }
    }
    return this.negate;
  }
}

class AndNode {
  private filters: AtomicSceneFilter[];
  private nodes: OrNode[];

  constructor() {
    this.filters = [];
    this.nodes = [];
  }

  public parse(s: string): string {
    // parse until an "or", ")" or end of string is reached, then return s (consuming a possible ')'?)
    for (;;) {
      let negate = false;

      if (s[0] === '!') {
        negate = true;
        s = s.slice(1);
      }

      if (s[0] === '(') {
        let subtree = new OrNode(negate);
        s = subtree.parse(s.slice(1)).trimStart();
        this.nodes.push(subtree);
        if (s === '' || s[0] !== ')') {
          // error! Closing bracket missing
          console.warn('missing closing bracket');
          return '';
        }
        s = s.slice(1).trimStart();

        continue;
      }

      let f = new AtomicSceneFilter(negate);
      s = f.parse(s).trimStart();
      this.filters.push(f);
      // remainder of a at this point:
      //  - something starting with an 'or ' or a ')' -> return
      //  - after a word -> continue, if something is left

      if (s === '' || s.startsWith('or ') || s[0] === ')') return s;
    }
  }

  public matches(s: Scene): boolean {
    for (const f of this.filters) {
      if (!f.matches(s)) {
        return false;
      }
    }

    for (const n of this.nodes) {
      if (!n.matches(s)) {
        return false;
      }
    }
    return true;
  }
}

export class AtomicSceneFilter {
  constructor(private readonly negate: boolean) {}

  public matches(s: Scene): boolean {
    return this.test(s) != this.negate;
  }

  // this will be overwritten with the actual function
  // by parse
  private test(s: Scene): boolean {
    return true; // an empty filter is true for all scenes
  }

  public parse(s: string): string {
    if (!s || s[0] === ')' || s.startsWith('or ')) return s;

    // Figure out the end of the first word
    let end = 0;
    while (end < s.length && s[end] !== ' ' && s[end] !== '(' && s[end] !== ')') {
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
      if (w.startsWith('actor:'))
        this.test = (s) => !!s.actors && s.actors.indexOf(w.slice(6)) !== -1;
      else if (w.startsWith('website:'))
        this.test = (s) => !!s.website && s.website.indexOf(w.slice(8)) !== -1;
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
    } else if(w[0] === '^') {
      // Partial filter: true if any of the tags contains the word
      const x = w.slice(1);
      if(x !== '')
        this.test = (s) => {
          for(const t of s.tags) {
            if(t.indexOf(x) !== -1)
             return true;
          }
          return false;
        }
    } else {
      // General tag filter: true if any of the tags equals the word
      this.test = (s) => s.tags.includes(w);
    }

    return s;
  }
}
