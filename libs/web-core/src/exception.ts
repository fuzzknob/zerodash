export class AppException {
	constructor(public message: string) {}
}

export class UnAuthenticatedException extends AppException {
	constructor(public message = 'Unauthenticated') {
		super(message)
	}
}

export class ValidationException extends AppException {
	constructor(
		public message = 'Validation Error',
		public errors?: Record<string, string[]>,
	) {
		super(message)
	}
}

export class NotFoundException extends AppException {
	constructor(public message = 'Not Found') {
		super(message)
	}
}
