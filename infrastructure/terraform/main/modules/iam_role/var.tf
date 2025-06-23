variable "tags" {
  type = object({
    Service     = string
    Environment = string
    Repository  = string
  })
}

variable "name" {
  type = string
}

variable "policy" {
  type = string
}

variable "assume_role_policy" {
  type = string
}