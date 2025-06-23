resource "aws_iam_role" "role" {
  name = var.name
  assume_role_policy = var.assume_role_policy

  tags = merge(var.tags, {
    Name = var.name,
  })
}

resource "aws_iam_policy" "policy" {
  name        = var.name
  policy = var.policy

  tags = merge(var.tags, {
    Name = var.name,
  })
}

resource "aws_iam_role_policy_attachment" "attachment" {
  role       = aws_iam_role.role.name
  policy_arn = aws_iam_policy.policy.arn
}
