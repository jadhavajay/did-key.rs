using System;
using TestDidKey;

namespace TestDidKey
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            var res = DIDKey.GenerateNew("ED25519");
            Console.WriteLine("response is; {0}",res);
        }
    }
}
