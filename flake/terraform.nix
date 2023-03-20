{ config, lib, options, specialArgs }:
let
  var = options.variable;
  # really should do TF_VAR so can arbitrary global suffixes and experimentation
  projet-name = "ggchain";
  tags = {
    tool = "terranix";
  };
in
rec {
  provider = {
    aws = {
      region = "eu-west-1";
    };
  };

  resource = {
    aws_s3_bucket = {
      terraform-backend = {
        bucket = "terraform-backend-${projet-name}";
        inherit tags;
      };
    };
  };

  # resource "aws_s3_bucket_acl" "example" {
  #   bucket = aws_s3_bucket.b.id
  #   acl    = "private"
  # }

}
