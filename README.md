# Rust Expression Calculator  

## 프로젝트 개요

- 언어: Rust (처음 사용하는 언어)
- 기간: 약 2주  
- 핵심 목표: 
  1. Rust 언어 문법·도구 체계를 빠르게 익히기  
  2. 문자열 계산기를 확장하여 간단한 미니 언어 엔진 개발
  3. 컴파일러 구조(lexer → parser → evaluator)를 직접 구현해보기  

Rust의 강점(enum, pattern matching, error handling)을 실제 문제 해결에 적용하기 위해 도전한 프로젝트입니다.


## 지원하는 기능 (Features)

### 정수 표현식
- 숫자: `1`, `42`, `1000`
- 사칙연산: +, -, *, /
- 괄호: `(1 + 2) * 3'
- 우선순위 자동 처리

### 변수(assignment)
- 번수 기능 : 변수에 값을 대입 및 이용하여 계산 가능

### 단항 마이너스(Unary -)
- -(음수) 사용 가능

### 내장 함수 호출
- 빌트인 함수 3개 제공 : max, min, abs

### REPL 모드
- 직접 입력하며 계산 가능한 인터렉티브 모드

### 에러 처리
- DivideByZero, UndefinedVariable 등

### 단위, 통합 테스트
- Lexer 단위 테스트
- Evaluator 통합 테스트 : 사칙연산, 괄호/우선순위, unary -, 변수 대입/사용, 빌트인 함수