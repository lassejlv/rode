/**
 * Rode JavaScript Runtime - Type Definitions
 *
 * Global APIs available in the Rode runtime environment.
 */

declare namespace Rode {
  /**
   * HTTP Request object passed to server handlers
   */
  interface Request {
    /** HTTP method (GET, POST, PUT, DELETE, etc.) */
    method: string
    /** Request URL path */
    url: string
    /** Request headers (when available) */
    headers?: Record<string, string>
    /** Request body (when available) */
    body?: string
  }

  /**
   * HTTP Response object returned from server handlers
   */
  interface Response {
    /** HTTP status code (200, 404, 500, etc.) */
    status: number
    /** Response body content */
    body: string
    /** Response headers (optional) */
    headers?: Record<string, string>
  }

  /**
   * HTTP request handler function type
   */
  type RequestHandler = (request: Request) => Response | string

  /**
   * Directory entry information
   */
  interface DirEntry {
    /** File or directory name */
    name: string
    /** True if this entry is a directory */
    isDirectory: boolean
    /** True if this entry is a file */
    isFile: boolean
  }

  /**
   * Password strength analysis result
   */
  interface PasswordStrength {
    /** Strength score (0-100) */
    score: number
    /** Strength level (weak, fair, good, strong) */
    level: string
    /** Array of feedback messages */
    feedback: string[]
  }

  /**
   * Password generation options
   */
  interface PasswordGenOptions {
    /** Include lowercase letters (default: true) */
    lowercase?: boolean
    /** Include uppercase letters (default: true) */
    uppercase?: boolean
    /** Include numbers (default: true) */
    numbers?: boolean
    /** Include symbols (default: true) */
    symbols?: boolean
    /** Exclude similar characters like 0/O, 1/l (default: false) */
    excludeSimilar?: boolean
  }

  /**
   * Password hashing and security utilities
   */
  namespace password {
    /**
     * Hash a password using bcrypt
     * @param password - Password to hash
     * @param rounds - Number of salt rounds (default: 12, min: 4, max: 20)
     * @returns Bcrypt hash string
     */
    function hash(password: string, rounds?: number): string

    /**
     * Verify a password against a hash
     * @param password - Password to verify
     * @param hash - Hash to verify against
     * @returns True if password matches hash
     */
    function verify(password: string, hash: string): boolean

    /**
     * Analyze password strength
     * @param password - Password to analyze
     * @returns Strength analysis result
     */
    function strength(password: string): PasswordStrength

    /**
     * Generate a secure random password
     * @param length - Password length (default: 16, min: 4, max: 128)
     * @param options - Generation options
     * @returns Generated password
     */
    function generate(length?: number, options?: PasswordGenOptions): string
  }

  /**
   * UUID generation and utilities
   */
  namespace uuid {
    /**
     * Generate a random UUID v4
     * @returns Random UUID v4 string
     */
    function v4(): string

    /**
     * Generate a time-based UUID v1
     * @returns Time-based UUID v1 string
     */
    function v1(): string

    /**
     * Generate a time-ordered UUID v7 (recommended)
     * @returns Time-ordered UUID v7 string
     */
    function v7(): string

    /**
     * Get the nil UUID (all zeros)
     * @returns Nil UUID string
     */
    function nil(): string

    /**
     * Parse and normalize a UUID string
     * @param uuid - UUID string to parse
     * @returns Normalized UUID string
     * @throws Error if UUID format is invalid
     */
    function parse(uuid: string): string

    /**
     * Validate a UUID string
     * @param uuid - UUID string to validate
     * @returns True if valid UUID format
     */
    function validate(uuid: string): boolean

    /**
     * Get the version of a UUID
     * @param uuid - UUID string to analyze
     * @returns Version number (1-5) or null if invalid
     */
    function version(uuid: string): number | null
  }

  /**
   * Path manipulation utilities
   */
  namespace path {
    /**
     * Join path segments together
     * @param paths - Path segments to join
     * @returns Joined path
     */
    function join(...paths: string[]): string

    /**
     * Resolve path segments to an absolute path
     * @param paths - Path segments to resolve
     * @returns Absolute path
     */
    function resolve(...paths: string[]): string

    /**
     * Get directory name of a path
     * @param path - Input path
     * @returns Directory name
     */
    function dirname(path: string): string

    /**
     * Get base name of a path
     * @param path - Input path
     * @param ext - Optional extension to remove
     * @returns Base name
     */
    function basename(path: string, ext?: string): string

    /**
     * Get file extension
     * @param path - Input path
     * @returns File extension (including the dot)
     */
    function extname(path: string): string

    /**
     * Check if path is absolute
     * @param path - Input path
     * @returns True if absolute
     */
    function isAbsolute(path: string): boolean

    /**
     * Normalize a path (resolve . and ..)
     * @param path - Input path
     * @returns Normalized path
     */
    function normalize(path: string): string

    /**
     * Get relative path from one path to another
     * @param from - Source path
     * @param to - Target path
     * @returns Relative path
     */
    function relative(from: string, to: string): string

    /** Path separator ('/' on Unix, '\' on Windows) */
    const sep: string

    /** PATH environment variable delimiter (':' on Unix, ';' on Windows) */
    const delimiter: string
  }

  /**
   * File system operations
   */
  namespace fs {
    /**
     * Read the contents of a file synchronously
     *
     * @param filename Path to the file to read
     * @returns The file contents as a string
     * @throws Error if the file cannot be read
     *
     * @example
     * ```js
     * const content = Rode.fs.readFile('config.json');
     * const config = JSON.parse(content);
     * ```
     */
    function readFile(filename: string): string

    /**
     * Write content to a file synchronously
     *
     * @param filename Path to the file to write
     * @param content Content to write to the file
     * @throws Error if the file cannot be written
     *
     * @example
     * ```js
     * const data = JSON.stringify({ key: 'value' });
     * Rode.fs.writeFile('output.json', data);
     * ```
     */
    function writeFile(filename: string, content: string): void

    /**
     * Check if a file or directory exists
     *
     * @param path Path to check
     * @returns True if the path exists, false otherwise
     *
     * @example
     * ```js
     * if (Rode.fs.exists('config.json')) {
     *   const config = Rode.fs.readFile('config.json');
     * }
     * ```
     */
    function exists(path: string): boolean

    /**
     * Create a directory
     *
     * @param path Directory path to create
     * @param recursive If true, create parent directories if they don't exist
     * @throws Error if the directory cannot be created
     *
     * @example
     * ```js
     * Rode.fs.mkdir('logs');
     * Rode.fs.mkdir('nested/deep/path', true);
     * ```
     */
    function mkdir(path: string, recursive?: boolean): void

    /**
     * Remove a file or directory
     *
     * @param path Path to remove
     * @param recursive If true, remove directories and their contents recursively
     * @throws Error if the path cannot be removed
     *
     * @example
     * ```js
     * Rode.fs.remove('temp.txt');
     * Rode.fs.remove('temp-dir', true);
     * ```
     */
    function remove(path: string, recursive?: boolean): void

    /**
     * Read directory contents
     *
     * @param path Directory path to read
     * @returns Array of directory entries with name and type information
     * @throws Error if the directory cannot be read
     *
     * @example
     * ```js
     * const entries = Rode.fs.readDir('.');
     * entries.forEach(entry => {
     *   console.log(`${entry.name} - ${entry.isDirectory ? 'Dir' : 'File'}`);
     * });
     * ```
     */
    function readDir(path: string): DirEntry[]
  }

  /**
   * HTTP server operations
   */
  namespace http {
    /**
     * Start an HTTP server with the given request handler
     *
     * @param handler Function that processes incoming HTTP requests
     * @param port Port number to listen on (default: 8000)
     *
     * @example
     * ```js
     * Rode.http.serve((request) => {
     *   if (request.url === '/') {
     *     return { status: 200, body: 'Hello World!' };
     *   }
     *   return { status: 404, body: 'Not Found' };
     * }, 3000);
     * ```
     */
    function serve(handler: RequestHandler, port?: number): void
  }
}

/**
 * Console API for logging output
 */
declare namespace console {
  /**
   * Log messages to the console
   *
   * @param message Primary message to log
   * @param optionalParams Additional parameters to log
   *
   * @example
   * ```js
   * console.log('Hello', 'World', 42);
   * ```
   */
  function log(message?: any, ...optionalParams: any[]): void
}
